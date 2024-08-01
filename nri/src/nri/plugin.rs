// Copyright (c) 2024 Huawei Technologies Co.,Ltd. All rights reserved.
//
// isula-rust-extensions is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2.
// You may obtain a copy of Mulan PSL v2 at:
//         http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

use lazy_static::lazy_static;
use std::os::linux::net::SocketAddrExt;
use std::os::unix::fs::{FileTypeExt, PermissionsExt};
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::fs::{self, remove_file, Permissions};
use std::thread;
use crate::nri::error::{Result, Error};
use crate::nri::c_transfer::{self, NriUpdateContainersResponse};
use std::os::fd::{FromRawFd, IntoRawFd, RawFd};
use std::os::unix::net::{SocketAddr, UnixListener, UnixStream};
use std::sync::{Arc, Mutex, RwLock};
use crate::nri::mux;
use ttrpc::Server;
use crate::protocols::{nri, nri_ttrpc};

pub struct Plugin {
    mux: Arc<mux::Mux>,
    client: Arc<nri_ttrpc::PluginClient>,
    timeout: i64
}

impl Drop for Plugin {
    fn drop(&mut self) {
        self.mux.clone().close();
    }
}

const PLUGIN_SERVICE_CONN: u32 = 1;
const RUNTIME_SERVICE_CONN: u32 = 2;

lazy_static!{
    static ref PLUGINS: RwLock<HashMap<String, (Arc<Plugin>, Server)>> = RwLock::new(HashMap::new());
    static ref RUNTIME_CALLBACKS: RwLock<c_transfer::NriRuntimeCallbacks> = RwLock::new(
        c_transfer::NriRuntimeCallbacks { register_plugin: None, update_containers: None });
    static ref EXTERNAL_CONNECT_LISTENER: Mutex<Option<UnixListener>> = Mutex::new(None);
}

struct NriRuntimeService {
    plugin_id: String,
}
impl nri_ttrpc::Runtime for NriRuntimeService {
    fn register_plugin(
        &self,
        _ctx: &::ttrpc::TtrpcContext,
        _req: nri::RegisterPluginRequest
    ) -> ttrpc::Result<nri::Empty> {
        println!("isula_rust_extensions::register_plugin: runtime service registering plugin for {}...", self.plugin_id);

        let callbacks = RUNTIME_CALLBACKS.read().
            map_err(|e| ttrpc::Error::Others(format!("lock error: {}", e)))?.clone();
        if let Some(register_plugin) = callbacks.register_plugin {
            let c_plugin_id = std::ffi::CString::new(self.plugin_id.clone()).unwrap().into_raw();
            let c_req = Box::into_raw(Box::new(c_transfer::NriRegisterPluginRequest::from(&_req)));
            if register_plugin(c_plugin_id, c_req) != 0 {
                return Err(ttrpc::Error::Others(format!("register plugin for {} failed", self.plugin_id)));
            }
            let rep = nri::Empty::new();
            let _unused = unsafe { Box::from_raw(c_plugin_id) };
            let _unused = unsafe { Box::from_raw(c_req) };
            return Ok(rep)
        }
        return Err(ttrpc::Error::Others("register plugin callback not registered".to_string()));
    }

    fn update_containers(
        &self,
        _ctx: &::ttrpc::TtrpcContext,
        _req: nri::UpdateContainersRequest
    ) -> ttrpc::Result<nri::UpdateContainersResponse> {
        println!("isula_rust_extensions::update_containers: runtime service updating containers for {}...", self.plugin_id);

        let callbacks = RUNTIME_CALLBACKS.read().
            map_err(|e| ttrpc::Error::Others(format!("lock error: {}", e)))?.clone();
        if let Some(update_containers) = callbacks.update_containers {
            let c_plugin_id = std::ffi::CString::new(self.plugin_id.clone()).unwrap().into_raw();
            let c_req = Box::into_raw(Box::new(c_transfer::NriUpdateContainersRequest::from(&_req)));
            let mut c_resp: *mut NriUpdateContainersResponse = std::ptr::null_mut();
            if update_containers(c_plugin_id, c_req, &mut c_resp) != 0 {
                return Err(ttrpc::Error::Others(format!("update containers for {} failed", self.plugin_id)));
            }
            let resp = nri::UpdateContainersResponse::from(unsafe { &*c_resp });

            let _unused = unsafe { Box::from_raw(c_plugin_id) };
            let _unused = unsafe { Box::from_raw(c_req) };
            let _unused = unsafe { Box::from_raw(c_resp) };
            return Ok(resp);
        }
        return Err(ttrpc::Error::Others("update containers callback not registered".to_string()));
    }
}

fn create_dir_all_with_permissions<P: AsRef<Path>>(path: P, mode: u32) -> std::io::Result<()> {
    let path = path.as_ref();
    let mut current_path = PathBuf::new();

    for component in path.components() {
        current_path.push(component.as_os_str());
        if !current_path.exists() {
            fs::create_dir(&current_path)?;
            fs::set_permissions(&current_path, Permissions::from_mode(mode))?;
        }
    }

    Ok(())
}

fn remove_socket_file(socket_addr: &String) -> Result<()> {
    match fs::metadata(socket_addr) {
        Ok(metadata) => {
            if metadata.file_type().is_socket() {
                remove_file(socket_addr)?;
            } else {
                return Err(Error::InvalidArgument(format!("{} is not a socket file", socket_addr)));
            }
        },
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => (),
        Err(e) => return Err(Error::IOError(e.to_string())),
    }
    Ok(())
}

pub fn runtime_service_init(callbacks: c_transfer::NriRuntimeCallbacks) -> Result<()> {
    let mut runtime_callbacks = RUNTIME_CALLBACKS.write().unwrap();
    if runtime_callbacks.register_plugin.is_some() || runtime_callbacks.update_containers.is_some() {
        return Err(Error::Other("runtime service already initialized".to_string()));
    }

    if callbacks.register_plugin.is_none() || callbacks.update_containers.is_none() {
        return Err(Error::InvalidArgument("callbacks not set".to_string()));
    }

    *runtime_callbacks = callbacks;
    Ok(())
}

pub fn runtime_service_destroy() {
    let mut plugins = PLUGINS.write().unwrap();
    for (_unused, (_, server)) in plugins.drain() {
        server.shutdown();
    }
}

// *RUNTIME_SERVER.lock().unwrap() = Some(server);
// plugin connect is implemented for the adjustment of the plugin connection
// from plugin side((https://github.com/containerd/nri).
// the plugin use the peer fd for data transfer in a mux way.
// plugin writes data to the peer fd in the format of hdr{connId, cnt} + ttrpc data.
// After we read the data in the local fd, we should parse it first and then do the ttrpc call.
// For ttrpc implemetation:
//   for Runtime Service: we use one unix socket to listen to the requests for per plugin.
//       We have a connection to the unix socket for each plugin to write and read to the service.
//   for Plugin Client: we create a socket pair for each plugin to write and read.
//       One end is to receive data from container runtime and transfer to ttrpc data
//       The other end is to handle these data and send to plugin.
pub fn connect(plugin_id: &String, local_fd: RawFd, timeout: i64) -> Result<()> {
    let trunk_stream = unsafe { UnixStream::from_raw_fd(local_fd) };

    let mut plugins = PLUGINS.write()
        .map_err(|e| Error::Other(format!("lock error: {}", e)))?;

    // socket1 will be added to conn & socket2 will be used to create a client
    let (socket1, socket2) = UnixStream::pair()
        .map_err(|e| Error::IOError(format!("create socket pair error: {}", e)))?;

    // Register one runtime service of container runtime for each plugin.
    let runtime_socket_addr = ("/var/run/nri/").to_string() + plugin_id + ".sock";
    let nri = Box::new(NriRuntimeService {
        plugin_id: plugin_id.clone()
    }) as Box<dyn nri_ttrpc::Runtime + Send + Sync>;
    let nri = Arc::new(nri);
    let nriservice = nri_ttrpc::create_runtime(nri);

    // for runtime service, we use an abstract unix socket
    let mut server = Server::new()
        .bind(("unix://@".to_string() + &runtime_socket_addr).as_str())
        .map_err(|e| Error::Other(format!("Bind error: {}", e)))?
        .register_service(nriservice);

    let plugin = Plugin {
        mux: Arc::new(mux::Mux::new(trunk_stream)),
        client: Arc::new(nri_ttrpc::PluginClient::new(ttrpc::Client::new(socket2.into_raw_fd())
            .map_err(|e| Error::TtrpcError(format!("create client error: {}", e)))?)),
        timeout: timeout
    };

    plugin.mux.clone().add_conn(PLUGIN_SERVICE_CONN, socket1)?;

    server.start().map_err(|e| Error::Other(format!("start error: {}", e)))?;

    if SocketAddr::from_abstract_name(runtime_socket_addr.as_bytes())
        .is_ok_and(|addr| UnixStream::connect_addr(&addr)
            .is_ok_and(|stream| plugin.mux.clone().add_conn(RUNTIME_SERVICE_CONN, stream)
                .is_ok())) {
        plugin.mux.clone().trunk_reader();
        plugins.insert(plugin_id.clone(), (Arc::new(plugin), server));
    } else {
        server.shutdown();
        return Err(Error::IOError("connect runtime service error".to_string()));
    }

    Ok(())
}

pub fn disconnect(plugin_id: &String) -> Result<()> {
    let mut plugins = PLUGINS.write()
        .map_err(|e| Error::Other(format!("lock error: {}", e)))?;
    if let Some((_, server)) = plugins.remove(plugin_id) {
        server.shutdown();
    }
    Ok(())
}

pub fn external_service_start(socket_addr: &String, callback: Option<c_transfer::NriExternalConnectCallback>) -> Result<()> {
    if (*EXTERNAL_CONNECT_LISTENER.lock().unwrap()).is_some() {
        return Err(Error::Other("external service already started".to_string()));
    }

    if callback.is_none() {
        return Err(Error::InvalidArgument("external service callback not set".to_string()));
    }

    remove_socket_file(socket_addr)?;

    if let Some(parent_dir) = Path::new(socket_addr).parent() {
        create_dir_all_with_permissions(parent_dir, 0o700)?;
    }

    let listener = UnixListener::bind(socket_addr)?;

    fs::set_permissions(socket_addr, Permissions::from_mode(0o600))
        .map_err(|e| Error::IOError(e.to_string()))?;

    *EXTERNAL_CONNECT_LISTENER.lock().unwrap() = Some(listener.try_clone()?);

    thread::spawn(move || {
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let fd = stream.into_raw_fd();
                    if callback.unwrap()(fd) != 0 {
                        unsafe {libc::close(fd)};
                        println!("isula_rust_extensions::external_service connect callback failed");
                    }
                },
                Err(_) => {
                    println!("isula_rust_extensions::external_service exited");
                    break;
                }
            }
        }
    });
    Ok(())
}

pub fn external_service_shutdown() {
    if let Some(listener) = EXTERNAL_CONNECT_LISTENER.lock().unwrap().take() {
        unsafe {libc::shutdown(listener.into_raw_fd(), libc::SHUT_RDWR)};
    }
}

fn plugin_get(plugin_id: &String) -> Result<Arc<Plugin>> {
    let plugins = PLUGINS.read().map_err(|e| Error::Other(format!("lock error: {}", e)))?;
    if plugins.contains_key(plugin_id) {
        Ok(plugins.get(plugin_id).unwrap().0.clone())
    } else {
        Err(Error::Other("client not found".to_string()))
    }
}

pub fn configure(plugin_id: &String, req: &nri::ConfigureRequest) -> Result<nri::ConfigureResponse> {
    let plugin = plugin_get(plugin_id)?;
    let res = plugin.client
        .configure(ttrpc::context::with_timeout(plugin.timeout), req)
        .map_err(|e| Error::TtrpcError(format!("configure error: {}", e)))?;
    Ok(res)
}

pub fn synchronize(plugin_id: &String, req: &nri::SynchronizeRequest) -> Result<nri::SynchronizeResponse> {
    let plugin = plugin_get(plugin_id)?;
    let res = plugin.client
        .synchronize(ttrpc::context::with_timeout(plugin.timeout), req)
        .map_err(|e| Error::TtrpcError(format!("synchronize error: {}", e)))?;
    Ok(res)
}

pub fn shutdown(plugin_id: &String) -> Result<()> {
    let plugin = plugin_get(plugin_id)?;
    plugin.client.shutdown(ttrpc::context::with_timeout(plugin.timeout), &nri::Empty::new())
        .map_err(|e| Error::TtrpcError(format!("shutdown error: {}", e)))?;
    Ok(())
}

pub fn create_container(plugin_id: &String, req: &nri::CreateContainerRequest) -> Result<nri::CreateContainerResponse> {
    let plugin = plugin_get(plugin_id)?;
    let res = plugin.client
        .create_container(ttrpc::context::with_timeout(plugin.timeout), req)
        .map_err(|e| Error::TtrpcError(format!("create container error: {}", e)))?;
    Ok(res)
}

pub fn update_container(plugin_id: &String, req: &nri::UpdateContainerRequest) -> Result<nri::UpdateContainerResponse> {
    let plugin = plugin_get(plugin_id)?;
    let res = plugin.client
        .update_container(ttrpc::context::with_timeout(plugin.timeout), req)
        .map_err(|e| Error::TtrpcError(format!("start container error: {}", e)))?;
    Ok(res)
}

pub fn stop_container(plugin_id: &String, req: &nri::StopContainerRequest) -> Result<nri::StopContainerResponse> {
    let plugin = plugin_get(plugin_id)?;
    let res = plugin.client
        .stop_container(ttrpc::context::with_timeout(plugin.timeout), req)
        .map_err(|e| Error::TtrpcError(format!("stop container error: {}", e)))?;
    Ok(res)
}

pub fn state_change(plugin_id: &String, req: &nri::StateChangeEvent) -> Result<()> {
    let plugin = plugin_get(plugin_id)?;
    plugin.client.state_change(ttrpc::context::with_timeout(plugin.timeout), req)
        .map_err(|e| Error::TtrpcError(format!("state change error: {}", e)))?;
    Ok(())
}
