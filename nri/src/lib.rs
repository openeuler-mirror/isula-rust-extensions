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
pub mod protocols;
pub mod nri;

use nri::{c_transfer, plugin};
use std::os::raw::{c_char, c_int};
use isula_common::isula_data_types::to_string;

#[no_mangle]
pub extern "C" fn nri_runtime_service_init(callbacks: c_transfer::NriRuntimeCallbacks) -> c_int {
    println!("isula-rust-extensions::nri_runtime_service_init");
    if let Err(e) = plugin::runtime_service_init(callbacks) {
        println!("isula-rust-extensions::nri_runtime_service failed: {}", e);
        return -1;
    }

    println!("isula-rust-extensions::nri_runtime_service_init success");
    0
}

#[no_mangle]
pub extern "C" fn nri_runtime_service_destroy() {
    println!("isula-rust-extensions::nri_runtime_service_destroy");

    plugin::runtime_service_destroy();

    println!("isula-rust-extensions::nri_runtime_service_destroy success");
}

#[no_mangle]
pub extern "C" fn nri_plugin_connect(plugin_id: *const c_char, local_fd: c_int, timeout: i64) -> c_int {
    if plugin_id.is_null() {
        return -1;
    }
    let r_plugin_id = to_string(plugin_id);
    println!("isula-rust-extensions::nri_plugin_connect with::{}", r_plugin_id);
    if let Err(e) = plugin::connect(&r_plugin_id, local_fd, timeout) {
        println!("isula-rust-extensions::nri_plugin_connect failed: {}", e);
        return -1;
    }

    println!("isula-rust-extensions::nri_plugin_connect success");
    0
}

#[no_mangle]
pub extern "C" fn nri_plugin_disconnect(plugin_id: *const c_char) -> c_int {
    if plugin_id.is_null() {
        return -1;
    }
    let r_plugin_id = to_string(plugin_id);
    println!("isula-rust-extensions::nri_plugin_disconnect with::{}", r_plugin_id);

    if let Err(e) = plugin::disconnect(&r_plugin_id) {
        println!("isula-rust-extensions::nri_plugin_disconnect failed: {}", e);
        return -1;
    }

    println!("isula-rust-extensions::nri_plugin_disconnect success");
    0
}

#[no_mangle]
pub extern "C" fn nri_external_service_start(socket_addr: *const c_char,
                                             callback: Option<c_transfer::NriExternalConnectCallback>) -> c_int {
    if socket_addr.is_null() {
        return -1;
    }
    let r_socket_addr = to_string(socket_addr);
    println!("isula-rust-extensions::nri_external_service_start with::{}", r_socket_addr);
    if let Err(e) = plugin::external_service_start(&r_socket_addr, callback) {
        println!("isula-rust-extensions::nri_external_service_start failed: {}", e);
        return -1;
    }

    println!("isula-rust-extensions::nri_external_service_start success");
    0
}

#[no_mangle]
pub extern "C" fn nri_external_service_shutdown() {
    println!("isula-rust-extensions::nri_external_service_shutdown");

    plugin::external_service_shutdown();

    println!("isula-rust-extensions::nri_external_service_shutdown success");
}

#[no_mangle]
pub extern "C" fn nri_plugin_configure(plugin_id: *const c_char,
    req: *const c_transfer::NriConfigureRequest,
    resp: *mut *const c_transfer::NriConfigureResponse
) -> c_int {
    if plugin_id.is_null() || req.is_null() || resp.is_null() {
        return -1;
    }
    let r_plugin_id = to_string(plugin_id);
    let c_req = unsafe { req.as_ref() }.unwrap();
    let r_req: protocols::nri::ConfigureRequest = protocols::nri::ConfigureRequest::from(c_req);
    println!("isula-rust-extensions::nri_plugin_configure with::{}", r_plugin_id);

    match plugin::configure(&r_plugin_id, &r_req) {
        Ok(r_resp) => {
            let c_resp = c_transfer::NriConfigureResponse::from(&r_resp);
            unsafe {
                *resp = Box::into_raw(Box::new(c_resp));
            }
        },
        Err(e) => {
            println!("isula-rust-extensions::nri_plugin_configure failed: {}", e);
            return -1;
        }
    }
    0
}

#[no_mangle]
pub extern "C" fn nri_plugin_synchronize(plugin_id: *const c_char,
    req: *const c_transfer::NriSynchronizeRequest,
    resp: *mut *const c_transfer::NriSynchronizeResponse
) -> c_int {
    if plugin_id.is_null() || req.is_null() || resp.is_null() {
        return -1;
    }
    let r_plugin_id = to_string(plugin_id);
    let c_req = unsafe { req.as_ref() }.unwrap();
    let r_req: protocols::nri::SynchronizeRequest = protocols::nri::SynchronizeRequest::from(c_req);
    println!("isula-rust-extensions::nri_plugin_synchronize with::{}", r_plugin_id);

    match plugin::synchronize(&r_plugin_id, &r_req) {
        Ok(r_resp) => {
            let c_resp = c_transfer::NriSynchronizeResponse::from(&r_resp);
            unsafe {
                *resp = Box::into_raw(Box::new(c_resp));
            }
        },
        Err(e) => {
            println!("isula-rust-extensions::nri_plugin_synchronize failed: {}", e);
            return -1;
        }
    }
    0
}

#[no_mangle]
pub extern "C" fn nri_plugin_shutdown(plugin_id: *const c_char
) -> c_int {
    if plugin_id.is_null() {
        return -1;
    }
    let r_plugin_id = to_string(plugin_id);
    println!("isula-rust-extensions::nri_plugin_shutdown with::{}", r_plugin_id);

    match plugin::shutdown(&r_plugin_id) {
        Ok(_) => {},
        Err(e) => {
            println!("isula-rust-extensions::nri_plugin_shutdown failed: {}", e);
            return -1;
        }
    }
    0
}

#[no_mangle]
pub extern  "C" fn nri_plugin_create_container(plugin_id: *const c_char,
    req: *const c_transfer::NriCreateContainerRequest,
    resp: *mut *const c_transfer::NriCreateContainerResponse
) -> c_int {
    if plugin_id.is_null() || req.is_null() || resp.is_null() {
        return -1;
    }
    let r_plugin_id = to_string(plugin_id);
    let c_req = unsafe { req.as_ref() }.unwrap();
    let r_req: protocols::nri::CreateContainerRequest = protocols::nri::CreateContainerRequest::from(c_req);
    println!("isula-rust-extensions::nri_plugin_create_container with::{}", r_plugin_id);

    match plugin::create_container(&r_plugin_id, &r_req) {
        Ok(r_resp) => {
            let c_resp = c_transfer::NriCreateContainerResponse::from(&r_resp);
            unsafe {
                *resp = Box::into_raw(Box::new(c_resp));
            }
        },
        Err(e) => {
            println!("isula-rust-extensions::nri_plugin_create_container failed: {}", e);
            return -1;
        }
    }
    0
}

#[no_mangle]
pub extern "C" fn nri_plugin_update_container(plugin_id: *const c_char,
    req: *const c_transfer::NriUpdateContainerRequest,
    resp: *mut *const c_transfer::NriUpdateContainerResponse
) -> c_int {
    if plugin_id.is_null() || req.is_null() || resp.is_null() {
        return -1;
    }
    let r_plugin_id = to_string(plugin_id);
    let c_req = unsafe { req.as_ref() }.unwrap();
    let r_req: protocols::nri::UpdateContainerRequest = protocols::nri::UpdateContainerRequest::from(c_req);
    println!("isula-rust-extensions::nri_plugin_update_container with::{}", r_plugin_id);

    match plugin::update_container(&r_plugin_id, &r_req) {
        Ok(r_resp) => {
            let c_resp = c_transfer::NriUpdateContainerResponse::from(&r_resp);
            unsafe {
                *resp = Box::into_raw(Box::new(c_resp));
            }
        },
        Err(e) => {
            println!("isula-rust-extensions::nri_plugin_update_container failed: {}", e);
            return -1;
        }
    }
    0
}

#[no_mangle]
pub extern "C" fn nri_plugin_stop_container(plugin_id: *const c_char,
    req: *const c_transfer::NriStopContainerRequest,
    resp: *mut *const c_transfer::NriStopContainerResponse
) -> c_int {
    if plugin_id.is_null() || req.is_null() || resp.is_null() {
        return -1;
    }
    let r_plugin_id = to_string(plugin_id);
    let c_req = unsafe { req.as_ref() }.unwrap();
    let r_req: protocols::nri::StopContainerRequest = protocols::nri::StopContainerRequest::from(c_req);
    println!("isula-rust-extensions::nri_plugin_stop_container with::{}", r_plugin_id);

    match plugin::stop_container(&r_plugin_id, &r_req) {
        Ok(r_resp) => {
            let c_resp = c_transfer::NriStopContainerResponse::from(&r_resp);
            unsafe {
                *resp = Box::into_raw(Box::new(c_resp));
            }
        },
        Err(e) => {
            println!("isula-rust-extensions::nri_plugin_stop_container failed: {}", e);
            return -1;
        }
    }
    0
}

#[no_mangle]
pub extern "C" fn nri_plugin_state_change(plugin_id: *const c_char,
    req: *const c_transfer::NriStateChangeEvent
) -> c_int {
    if plugin_id.is_null() || req.is_null() {
        return -1;
    }
    let r_plugin_id = to_string(plugin_id);
    let c_req = unsafe { req.as_ref() }.unwrap();
    let r_req: protocols::nri::StateChangeEvent = protocols::nri::StateChangeEvent::from(c_req);
    println!("isula-rust-extensions::nri_plugin_state_change with::{}", r_plugin_id);

    match plugin::state_change(&r_plugin_id, &r_req) {
        Ok(_) => {},
        Err(e) => {
            println!("isula-rust-extensions::nri_plugin_state_change failed: {}", e);
            return -1;
        }
    }
    0
}
