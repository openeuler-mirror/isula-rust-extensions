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


use std::ffi::CString;
use std::os::raw::{c_char, c_void};
use isula_common::isula_data_types::{ByteArray, MapStringBytes, MapStringString};
use isula_common::isula_data_types::to_string;
use isula_common::isula_data_types::{vec_to_c_char_ptr_ptr, c_char_ptr_ptr_to_vec};
use isula_common::isula_data_types::{vec_to_double_ptr, double_ptr_to_vec};
use isula_common::isula_data_types::u64_to_prost_timestamp;
use isula_common::isula_data_types::prost_timestamp_to_u64;
use prost::Message;
use crate::controller::client::sandbox::containerd::types as sandbox;
use crate::controller::client::sandbox::containerd::services::sandbox::v1 as sandbox_services;

#[repr(C)]
pub struct SandboxMount {
    type_: *const c_char,
    source: *const c_char,
    destination: *const c_char,
    options: *const *const c_char,
    options_len: usize,
    residual: *const c_void,
}

impl From<&SandboxMount> for sandbox::Mount {
    fn from(req: &SandboxMount) -> Self {
        let mut r_req = sandbox::Mount::default();
        r_req.target = to_string(req.destination);
        r_req.r#type = to_string(req.type_);
        r_req.source = to_string(req.source);
        r_req.options = c_char_ptr_ptr_to_vec(req.options, req.options_len);
        r_req
    }
}

impl From<&sandbox::Mount> for SandboxMount {
    fn from(req: &sandbox::Mount) -> Self {
        let (options, options_len) = vec_to_c_char_ptr_ptr(&req.options);
        let r_req = SandboxMount {
            destination: CString::new(req.target.as_str()).unwrap().into_raw(),
            type_: CString::new(req.r#type.as_str()).unwrap().into_raw(),
            source: CString::new(req.source.as_str()).unwrap().into_raw(),
            options: options,
            options_len: options_len,
            residual: std::ptr::null(),
        };
        r_req
    }
}

#[repr(C)]
pub struct SandboxSandboxRuntime {
    name: *const c_char,
    options: *const u8,
    options_len: usize,
}

impl From<&SandboxSandboxRuntime> for sandbox::sandbox::Runtime {
    fn from(req: &SandboxSandboxRuntime) -> Self {
        let mut r_req = sandbox::sandbox::Runtime::default();
        r_req.name = to_string(req.name);
        if req.options.is_null() || req.options_len == 0 {
            r_req.options = None;
        } else {
            let req_options = unsafe { std::slice::from_raw_parts(req.options, req.options_len) };
            let any_options = match prost_types::Any::decode(req_options) {
                Ok(options) => Some(options),
                Err(e) => {
                    println!("Failed to decode options, {:?}", e);
                    None
                }
            };
            r_req.options = any_options;
        }
        r_req
    }
}

impl From<&sandbox::sandbox::Runtime> for SandboxSandboxRuntime {
    fn from(req: &sandbox::sandbox::Runtime) -> Self {
        let options = req.options.clone().unwrap_or_else(|| prost_types::Any::default());
        let mut buf = Vec::new();
        options.encode(&mut buf).unwrap_or_else(|e| {
            println!("Failed to encode options, {:?}", e);
        });
        let r_req = SandboxSandboxRuntime {
            name: CString::new(req.name.as_str()).unwrap().into_raw(),
            options_len: buf.len(),
            options: Box::into_raw(buf.into_boxed_slice()) as *const u8,
        };
        r_req
    }
}

#[repr(C)]
pub struct SandboxSandbox {
    sandbox_id: *const c_char,
    runtime: *const SandboxSandboxRuntime,
    spec: *const u8,
    spec_len: usize,
    labels: *const MapStringString,
    created_at: u64,
    updated_at: u64,
    extensions: *const MapStringBytes,
    sandboxer: *const c_char,
    residual: *const c_void,
}

impl From<&SandboxSandbox> for sandbox::Sandbox {
    fn from(req: &SandboxSandbox) -> Self {
        let mut r_req = sandbox::Sandbox::default();
        r_req.sandbox_id = to_string(req.sandbox_id);
        match unsafe { req.runtime.as_ref() } {
            Some(runtime) => {
                r_req.runtime = Some(sandbox::sandbox::Runtime::from(&*runtime));
            }
            None => {
                r_req.runtime = None;
            }
        }
        if req.spec.is_null() || req.spec_len == 0 {
            r_req.spec = None;
        } else {
            r_req.spec = Some(prost_types::Any::from(&ByteArray::new(req.spec, req.spec_len)));
        }
        if req.labels.is_null() {
            r_req.labels = std::collections::HashMap::new();
        } else {
            r_req.labels = unsafe { <std::collections::HashMap<String, String>>::from(&*req.labels) };
        }
        r_req.created_at = Some(u64_to_prost_timestamp(req.created_at));
        r_req.updated_at = Some(u64_to_prost_timestamp(req.updated_at));
        if req.extensions.is_null() {
            r_req.extensions = std::collections::HashMap::new();
        } else {
            r_req.extensions = unsafe { <std::collections::HashMap<String, prost_types::Any>>::from(&*req.extensions) };
        }
        r_req.sandboxer = to_string(req.sandboxer);
        r_req
    }
}

impl From<&sandbox::Sandbox> for SandboxSandbox {
    fn from(req: &sandbox::Sandbox) -> Self {
        let spec_byte_array = match &req.spec {
            Some(spec) => {
                ByteArray::from(spec)
            },
            None => ByteArray::new(std::ptr::null(), 0)
        };
        let created_at = match &req.created_at {
            Some(timestamp) => prost_timestamp_to_u64(timestamp),
            None => 0
        };
        let update_at = match &req.updated_at {
            Some(timestamp) => prost_timestamp_to_u64(timestamp),
            None => 0
        };
        let r_req = SandboxSandbox {
            sandbox_id: CString::new(req.sandbox_id.as_str()).unwrap().into_raw(),
            runtime: match &req.runtime {
                Some(rt) => Box::into_raw(Box::new(SandboxSandboxRuntime::from(rt))),
                None => std::ptr::null(),
            },
            spec: spec_byte_array.data,
            spec_len: spec_byte_array.len,
            labels: Box::into_raw(Box::new(MapStringString::from(&req.labels))),
            created_at: created_at,
            updated_at: update_at,
            extensions: Box::into_raw(Box::new(MapStringBytes::from(&req.extensions))),
            sandboxer: CString::new(req.sandboxer.as_str()).unwrap().into_raw(),
            residual: std::ptr::null(),
        };
        r_req
    }
}

#[repr(C)]
// order of the data structure is incorrect
pub struct SandboxCreateRequest {
    sandbox_id: *const c_char,
    rootfs: *const *const SandboxMount,
    rootfs_len: usize,
    options: *const u8,
    options_len: usize,
    netns_path: *const c_char,
    annotations: *const MapStringString,
    sandbox: *const SandboxSandbox,
    sandboxer: *const c_char,
    residual: *const c_void,
}

impl From<&SandboxCreateRequest> for sandbox_services::ControllerCreateRequest {
    fn from(req: &SandboxCreateRequest) -> Self {
        let mut r_req = sandbox_services::ControllerCreateRequest::default();
        r_req.sandbox_id = to_string(req.sandbox_id);
        r_req.rootfs = double_ptr_to_vec(req.rootfs, req.rootfs_len);
        if req.options.is_null() || req.options_len == 0 {
            r_req.options = None;
        } else {
            let encoded = unsafe { std::slice::from_raw_parts(req.options, req.options_len) };
            let any = prost_types::Any {
                type_url: "".to_string(), // TODO: fill in the type url
                value: encoded.to_vec(),
            };
            r_req.options = Some(any);
        }
        r_req.netns_path = to_string(req.netns_path);
        if req.annotations.is_null() {
            r_req.annotations = std::collections::HashMap::new();
        } else {
            r_req.annotations = unsafe { <std::collections::HashMap<String, String>>::from(&*req.annotations) };
        }
        match unsafe { req.sandbox.as_ref() } {
            Some(sandbox) => {
                r_req.sandbox = Some(sandbox::Sandbox::from(&*sandbox));
            }
            None => {
                r_req.sandbox = None;
            }
        }
        r_req.sandboxer = to_string(req.sandboxer);
        r_req
    }
}

impl From<&sandbox_services::ControllerCreateRequest> for SandboxCreateRequest {
    fn from(req: &sandbox_services::ControllerCreateRequest) -> Self {
        let (rootfs, rootfs_len) = vec_to_double_ptr(&req.rootfs);
        let (options_ptr, options_len) = match &req.options {
            Some(options) => {
                let mut buf = Vec::new();
                match options.encode(&mut buf) {
                    Ok(_) => {
                        let len = buf.len();
                        (Box::into_raw(buf.into_boxed_slice()) as *const u8, len)
                    },
                    Err(e) => {
                        println!("Failed to encode options, {:?}", e);
                        (std::ptr::null(), 0)
                    }
                }
            },
            None => (std::ptr::null(), 0)
        };
        let r_req = SandboxCreateRequest {
            sandbox_id: CString::new(req.sandbox_id.as_str()).unwrap().into_raw(),
            rootfs: rootfs,
            rootfs_len: rootfs_len,
            options: options_ptr,
            options_len: options_len,
            netns_path: CString::new(req.netns_path.as_str()).unwrap().into_raw(),
            annotations: Box::into_raw(Box::new(MapStringString::from(&req.annotations))),
            sandbox: match &req.sandbox {
                Some(sandbox) => Box::into_raw(Box::new(SandboxSandbox::from(sandbox))),
                None => std::ptr::null(),
            },
            sandboxer: CString::new(req.sandboxer.as_str()).unwrap().into_raw(),
            residual: std::ptr::null(),
        };
        r_req
    }
}

#[repr(C)]
pub struct SandboxCreateResponse {
    pub sandbox_id: *const c_char,
    residual: *const c_void,
}

impl From<&SandboxCreateResponse> for sandbox_services::ControllerCreateResponse {
    fn from(req: &SandboxCreateResponse) -> Self {
        let mut r_req = sandbox_services::ControllerCreateResponse::default();
        r_req.sandbox_id = to_string(req.sandbox_id);
        r_req
    }
}

impl From<&sandbox_services::ControllerCreateResponse> for SandboxCreateResponse {
    fn from(req: &sandbox_services::ControllerCreateResponse) -> Self {
        let r_req = SandboxCreateResponse {
            sandbox_id: CString::new(req.sandbox_id.as_str()).unwrap().into_raw(),
            residual: std::ptr::null(),
        };
        r_req
    }
}



