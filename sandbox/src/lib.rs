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
use std::os::raw::{c_char, c_int, c_void};
use std::sync::{ Arc, Mutex };
use lazy_static::lazy_static;
use tokio::runtime::Runtime;
use async_recursion::async_recursion;

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

pub type SandboxReadyCallback = extern "C" fn(*mut c_void);
pub type SandboxPendingCallback = extern "C" fn(*mut c_void);
pub type SandboxExitCallback = extern "C" fn(*mut c_void, *const sandbox_types::SandboxWaitResponse);

#[repr(C)]
pub struct SandboxWaitCallback {
    pub ready: SandboxReadyCallback,
    pub pending: SandboxPendingCallback,
    pub exit: SandboxExitCallback,
}

macro_rules! callback_execute {
    ($sandbox_id:ident, $callback:ident, $cb:ident, $ctx:ident, $rsp:expr) => {
        match $ctx.lock().unwrap().as_mut() {
            Some(ctx) => {
                ($callback.$cb)(*ctx as *mut c_void, $rsp);
            }
            None => {
                println!("Sandbox API: context is null, {:?}", $sandbox_id);
                ($callback.$cb)(std::ptr::null_mut(), $rsp);
            }
        }
    };
    ($sandbox_id:ident, $callback:ident, $cb:ident, $ctx:ident) => {
        match $ctx.lock().unwrap().as_mut()  {
            Some(ctx) => {
                ($callback.$cb)(*ctx as *mut c_void);
            }
            None => {
                println!("Sandbox API: context is null, {:?}", $sandbox_id);
                ($callback.exit)(std::ptr::null_mut(), std::ptr::null());
            }
        }
    };
}

const RETRY_INTERVAL: u64 = 5;
#[async_recursion]
async fn do_wait(
    mut client: client::Client,
    sandbox_id: String,
    req: ControllerWaitRequest,
    callback: SandboxWaitCallback,
    cb_ctx: Arc<Mutex<Option<&mut c_void>>>,
    retry: bool,
) {
    let mut retry = retry;
    // If this wait is for retry, set sandbox status back to ready if connection is alive
    if retry && client.is_connection_alive().await {
        callback_execute!(sandbox_id, callback, ready, cb_ctx);
        retry = false;
    }
    match client.wait(req.clone()).await {
        Ok(response) => {
            let mut r_rsp = sandbox_types::SandboxWaitResponse::new();
            r_rsp.from_controller(&response);
            r_rsp.sandbox_id = to_c_char_ptr(sandbox_id.as_str());
            println!("Sandbox API: Wait finished successful, {:?}", sandbox_id);
            callback_execute!(sandbox_id, callback, exit, cb_ctx, &r_rsp);
        }
        Err(e) => {
            println!("Sandbox API: Wait failed, {:?}, {:?}", sandbox_id, e);
            if !client.is_connection_alive().await && !retry {
                println!("Sandbox API: Connection is dead, {:?}", sandbox_id);
                callback_execute!(sandbox_id, callback, pending, cb_ctx);
            }
            sleep(Duration::from_secs(RETRY_INTERVAL)).await;
            do_wait(client.clone(), sandbox_id, req, callback, cb_ctx, true).await;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn sandbox_api_wait(
    handle: ControllerHandle,
    req: *const sandbox_types::SandboxWaitRequest,
    callback: SandboxWaitCallback,
    cb_ctx: *mut c_void,
) -> c_int {
    let controller_context = &mut *handle;
    let r_req = ControllerWaitRequest::from(&*req);
    println!("Sandbox API: Wait request: {:?}", r_req);
    match controller_context.get_client() {
        Some(client) => {
            let sandbox_id = r_req.sandbox_id.clone();
            let ctx: Arc<Mutex<Option<&mut c_void>>>= Arc::new(Mutex::new(cb_ctx.as_mut()));
            RT.spawn(do_wait(client.clone(), sandbox_id, r_req, callback, ctx, false));
            0
        }
        None => {
            println!("Sandbox API: Failed to execute sandbox API, client is None");
            -1
        }
    }
}
