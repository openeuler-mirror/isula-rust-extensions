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

#![crate_type = "dylib"]
mod controller;
mod datatype;
use controller::client;
use datatype::sandbox_types;
use tokio::time::{ sleep, Duration };
use std::os::raw::{c_char, c_int};
use lazy_static::lazy_static;
use tokio::runtime::Runtime;
use async_recursion::async_recursion;
use std::time::{SystemTime, UNIX_EPOCH};
use std::ffi::CStr;
use std::sync::Mutex;

use isula_common::isula_data_types::{ to_string, to_c_char_ptr };

use controller::client::sandbox::containerd::services::sandbox::v1::ControllerCreateRequest;
use controller::client::sandbox::containerd::services::sandbox::v1::ControllerStartRequest;
use controller::client::sandbox::containerd::services::sandbox::v1::ControllerPlatformRequest;
use controller::client::sandbox::containerd::services::sandbox::v1::ControllerStopRequest;
use controller::client::sandbox::containerd::services::sandbox::v1::ControllerWaitRequest;
use controller::client::sandbox::containerd::services::sandbox::v1::ControllerStatusRequest;
use controller::client::sandbox::containerd::services::sandbox::v1::ControllerShutdownRequest;
use controller::client::sandbox::containerd::services::sandbox::v1::ControllerMetricsRequest;
use controller::client::sandbox::containerd::services::sandbox::v1::ControllerUpdateRequest;


lazy_static! {
    static ref RT: Runtime = match Runtime::new() {
        Ok(rt) => rt,
        Err(e) => {
            println!("Sandbox API: Failed to create runtime, {:?}", e);
            // TODO: Fix this
            std::process::exit(1);
        }
    };
    static ref RUNTIME_MUTEX: Mutex<i32> = Mutex::new(0);
    static ref RETRY_WAIT_MUTEX: tokio::sync::Mutex<i32> = tokio::sync::Mutex::new(0);
}

#[repr(C)]
pub struct ControllerContext {
    sandboxer: String,
    address: String,
    client: Option<client::Client>,
}

pub type ControllerHandle = *mut ControllerContext;

impl ControllerContext {
    pub fn get_client(&mut self) -> Option<&mut client::Client>{
        if self.client.is_none() {
            let _rt_lock = RUNTIME_MUTEX.lock().unwrap();
            match RT.block_on(controller::client::Client::new(self.address.clone())) {
                Ok(client) => {
                    self.client = Some(client);
                }
                Err(e) => {
                    println!("Sandbox API: Failed to create controller client, {:?}", e);
                    self.client = None;
                }
            }
        }
        self.client.as_mut()
    }
}

macro_rules! sandbox_api_execute {
    ($context:ident, $request:ident, $rsp:ident, $method:ident) => {
        match $context.get_client() {
            Some(client) => {
                let _rt_lock = RUNTIME_MUTEX.lock().unwrap();
                match RT.block_on((*client).$method($request)) {
                    Ok(response) => {
                        (*$rsp).from_controller(&response);
                        0
                    }
                    Err(e) => {
                        println!("Sandbox API: Failed to execute sandbox API, {:?}", e);
                        -1
                    }
                }
            }
            None => {
                println!("Sandbox API: Failed to execute sandbox API, client is None");
                -1
            }
        }
    };
    ($context:ident, $request:ident, $method:ident) => {
        match $context.get_client() {
            Some(client) => {
                let _rt_lock = RUNTIME_MUTEX.lock().unwrap();
                match RT.block_on((*client).$method($request)) {
                    Ok(_) => 0,
                    Err(e) => {
                        println!("Sandbox API: Failed to execute sandbox API, {:?}", e);
                        -1
                    }
                }
            }
            None => {
                println!("Sandbox API: Failed to execute sandbox API, client is None");
                -1
            }
        }
    };
}

#[no_mangle]
pub extern "C" fn sandbox_api_build_controller(
    sandboxer: *const c_char,
    address: *const c_char,
) -> ControllerHandle {
    let r_sandboxer = to_string(sandboxer);
    let r_address = to_string(address);
    let mut controller_context = ControllerContext {
        sandboxer: r_sandboxer.clone(),
        address: r_address.clone(),
        client: None,
    };
    controller_context.get_client();
    println!(
        "Sandbox API: Controller created successfully for [sandboxer: {:?}, address: {:?}]",
        r_sandboxer, r_address
    );
    Box::into_raw(Box::new(controller_context))
}

#[no_mangle]
pub unsafe extern "C" fn sandbox_api_create(
    handle: ControllerHandle,
    req: *const sandbox_types::SandboxCreateRequest,
    rsp: *mut sandbox_types::SandboxCreateResponse,
) -> c_int {
    let controller_context = &mut *handle;
    let r_req = ControllerCreateRequest::from(&*req);
    println!("Sandbox API: Create request: {:?}", r_req);
    sandbox_api_execute!(controller_context, r_req, rsp, create)
}

#[no_mangle]
pub unsafe extern "C" fn sandbox_api_start(
    handle: ControllerHandle,
    req: *const sandbox_types::SandboxStartRequest,
    rsp: *mut sandbox_types::SandboxStartResponse,
) -> c_int {
    let controller_context = &mut *handle;
    let r_req= ControllerStartRequest::from(&*req);
    println!("Sandbox API: Start request: {:?}", r_req);
    sandbox_api_execute!(controller_context, r_req, rsp, start)
}

#[no_mangle]
pub unsafe extern "C" fn sandbox_api_platform(
    handle: ControllerHandle,
    req: *const sandbox_types::SandboxPlatformRequest,
    rsp: *mut sandbox_types::SandboxPlatformResponse,
) -> c_int {
    let controller_context = &mut *handle;
    let r_req = ControllerPlatformRequest::from(&*req);
    println!("Sandbox API: Platform request: {:?}", r_req);
    sandbox_api_execute!(controller_context, r_req, rsp, platform)
}

#[no_mangle]
pub unsafe extern "C" fn sandbox_api_stop(
    handle: ControllerHandle,
    req: *const sandbox_types::SandboxStopRequest,
) -> c_int {
    let controller_context = &mut *handle;
    let r_req = ControllerStopRequest::from(&*req);
    println!("Sandbox API: Stop request: {:?}", r_req);
    sandbox_api_execute!(controller_context, r_req, stop)
}

#[no_mangle]
pub unsafe extern "C" fn sandbox_api_status(
    handle: ControllerHandle,
    req: *const sandbox_types::SandboxStatusRequest,
    rsp: *mut sandbox_types::SandboxStatusResponse,
) -> c_int {
    let controller_context = &mut *handle;
    let r_req = ControllerStatusRequest::from(&*req);
    println!("Sandbox API: Status request: {:?}", r_req);
    sandbox_api_execute!(controller_context, r_req, rsp, status)
}

#[no_mangle]
pub unsafe extern "C" fn sandbox_api_shutdown(
    handle: ControllerHandle,
    req: *const sandbox_types::SandboxShutdownRequest,
) -> c_int {
    let controller_context = &mut *handle;
    let r_req = ControllerShutdownRequest::from(&*req);
    println!("Sandbox API: Shutdown request: {:?}", r_req);
    sandbox_api_execute!(controller_context, r_req, shutdown)
}

#[no_mangle]
pub unsafe extern "C" fn sandbox_api_metrics(
    handle: ControllerHandle,
    req: *const sandbox_types::SandboxMetricsRequest,
    rsp: *mut sandbox_types::SandboxMetricsResponse,
) -> c_int {
    let controller_context = &mut *handle;
    let r_req = ControllerMetricsRequest::from(&*req);
    println!("Sandbox API: Metrics request: {:?}", r_req);
    sandbox_api_execute!(controller_context, r_req, rsp, metrics)
}

#[no_mangle]
pub unsafe extern "C" fn sandbox_api_update(
    handle: ControllerHandle,
    req: *const sandbox_types::SandboxUpdateRequest,
) -> c_int {
    let controller_context = &mut *handle;
    let r_req = ControllerUpdateRequest::from(&*req);
    println!("Sandbox API: Update request: {:?}", r_req);
    sandbox_api_execute!(controller_context, r_req, update)
}

pub type SandboxReadyCallback = extern "C" fn(*const c_char);
pub type SandboxPendingCallback = extern "C" fn(*const c_char);
pub type SandboxExitCallback = extern "C" fn(*const c_char, *const sandbox_types::SandboxWaitResponse);

#[repr(C)]
pub struct SandboxWaitCallback {
    pub ready: SandboxReadyCallback,
    pub pending: SandboxPendingCallback,
    pub exit: SandboxExitCallback,
}

macro_rules! callback_execute {
    ($sandbox_id:ident, $callback:ident, $cb:ident, $rsp:expr) => {
        let sandbox_id_ptr = to_c_char_ptr($sandbox_id.as_str());
        ($callback.$cb)(sandbox_id_ptr, $rsp);
        unsafe {
            let _ = CStr::from_ptr(sandbox_id_ptr);
        }
    };
    ($sandbox_id:ident, $callback:ident, $cb:ident) => {
        let sandbox_id_ptr = to_c_char_ptr($sandbox_id.as_str());
        ($callback.$cb)(sandbox_id_ptr);
        unsafe {
            let _ = CStr::from_ptr(sandbox_id_ptr);
        }
    };
}

const RETRY_INTERVAL: u64 = 5;

pub async fn is_connection_alive(
    client: &mut client::Client,
    sandbox_id: &String,
    sandboxer: &String
) -> bool {
    let mut r_req = ControllerPlatformRequest::default();
    r_req.sandbox_id = sandbox_id.clone();
    r_req.sandboxer = sandboxer.clone();
    match (*client).platform(r_req).await {
        Ok(_) => true,
        Err(e) => {
            println!("Sandbox API: Failed to connect to client, {:?}, {:?}", sandbox_id, e);
            false
        }
    }
}

fn do_failed_exit_wait (
    exit_status: u32,
    message: String,
    sandbox_id: &String,
    callback: &SandboxWaitCallback,
){
    println!("Sandbox API: {:?}", message);

    let mut r_rsp = sandbox_types::SandboxWaitResponse::new();
    r_rsp.exit_status = exit_status;
    r_rsp.exited_at = SystemTime::now().duration_since(UNIX_EPOCH)
                                        .unwrap_or_default()
                                        .as_secs() as u64;
    r_rsp.sandbox_id = to_c_char_ptr(sandbox_id.as_str());
    callback_execute!(sandbox_id, callback, exit, &r_rsp);
    unsafe {
        let _ = CStr::from_ptr(r_rsp.sandbox_id);
    }
}

#[async_recursion]
async fn do_wait(
    mut client: client::Client,
    sandbox_id: String,
    req: ControllerWaitRequest,
    callback: SandboxWaitCallback,
) {
    let mut unavailable = false;

    match client.clone().wait(req.clone()).await {
        Ok(response) => {
            let mut r_rsp = sandbox_types::SandboxWaitResponse::new();
            r_rsp.from_controller(&response);
            r_rsp.sandbox_id = to_c_char_ptr(sandbox_id.as_str());
            println!("Sandbox API: Wait finished successful, {:?}", sandbox_id);
            callback_execute!(sandbox_id, callback, exit, &r_rsp);
            unsafe {
                let _ = CStr::from_ptr(r_rsp.sandbox_id);
            }
        }
        Err(e) => {
            let mut err_code = e.code();
            if format!("{:?}", e).contains("BrokenPipe") {
                err_code = tonic::Code::Unavailable;
            }
            match err_code {
                tonic::Code::Unavailable => {
                    println!("Sandbox API: Connection is unavailable, {:?}", sandbox_id);
                    callback_execute!(sandbox_id, callback, pending);
                    unavailable = true;
                }
                tonic::Code::NotFound => {
                    do_failed_exit_wait(e.code() as u32,
                        format!("The sandbox is not found, {:?}", sandbox_id),
                        &sandbox_id, &callback);
                }
                _ => {
                    do_failed_exit_wait(e.code() as u32,
                        format!("Connection failed, {:?}, {:?}", e, sandbox_id),
                        &sandbox_id, &callback);
                }
            }
        }
    }
    if unavailable {
        /* 
         * There is at most one async do_wait function which can acquire the mutex lock
         * and do the retry for connection, while other async do_wait functions will 
         * wait for the mutex to be unlocked.
        */
        match RETRY_WAIT_MUTEX.try_lock() {
            Ok(_) => {
                loop {
                    sleep(Duration::from_secs(RETRY_INTERVAL)).await;
                    if is_connection_alive(&mut client, &req.sandbox_id, &req.sandboxer).await {
                        break;
                    }
                }
            }
            Err(_) => {
                /* 
                 * Blocking other do_wait retries, and wait for the connection to be recovered.
                 */
                println!("Sandbox API: Wait block , {:?}", sandbox_id);
                _ = RETRY_WAIT_MUTEX.lock().await;
            }
        }

        println!("Sandbox API: Wait retry, {:?}", sandbox_id);
        callback_execute!(sandbox_id, callback, ready);
        do_wait(client, sandbox_id, req, callback).await;
    }
}

#[no_mangle]
pub unsafe extern "C" fn sandbox_api_wait(
    handle: ControllerHandle,
    req: *const sandbox_types::SandboxWaitRequest,
    callback: SandboxWaitCallback,
) -> c_int {
    let controller_context = &mut *handle;
    let r_req = ControllerWaitRequest::from(&*req);
    println!("Sandbox API: Wait request: {:?}", r_req);
    match controller_context.get_client() {
        Some(client) => {
            let sandbox_id = r_req.sandbox_id.clone();
            let _rt_lock = RUNTIME_MUTEX.lock().unwrap();
            RT.spawn(do_wait(client.clone(), sandbox_id, r_req, callback));
            0
        }
        None => {
            println!("Sandbox API: Failed to execute sandbox API, client is None");
            -1
        }
    }
}
