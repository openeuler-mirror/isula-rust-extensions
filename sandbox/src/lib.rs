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
use datatype::request;
use std::os::raw::{c_char, c_int, c_uint};
use lazy_static::lazy_static;
use tokio::runtime::Runtime;

use isula_common::isula_data_types::to_string;

use controller::client::sandbox::containerd::services::sandbox::v1::ControllerCreateRequest;

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
    req: *const request::SandboxCreateRequest,
    rsp: *mut request::SandboxCreateResponse,
) -> c_int {
    let controller_context = &mut *handle;
    let r_req = ControllerCreateRequest::from(&*req);
    match controller_context.get_client() {
        Some(client) => {
            match RT.block_on((*client).create(r_req)) {
                Ok(response) => {
                    (*rsp).sandbox_id = response.sandbox_id.as_ptr() as *const c_char;
                    0
                }
                Err(e) => {
                    println!("Sandbox API: Failed to create sandbox, {:?}", e);
                    -1
                }
            }
        }
        None => {
            println!("Sandbox API: Failed to create sandbox, client is None");
            -1
        }
    }
}
