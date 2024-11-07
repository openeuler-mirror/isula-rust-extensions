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
use isula_common::isula_data_types::double_ptr_to_vec;
use isula_common::isula_data_types::u64_to_prost_timestamp;
use isula_common::isula_data_types::prost_timestamp_to_u64;
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
            let any = prost_types::Any {
                type_url: "".to_string(), // TODO: fill in the type url
                value: req_options.to_vec(),
            };
            r_req.options = Some(any);
        }
        r_req
    }
}

impl From<&sandbox::sandbox::Runtime> for SandboxSandboxRuntime {
    fn from(req: &sandbox::sandbox::Runtime) -> Self {
        let options = req.options.clone().unwrap_or_else(|| prost_types::Any::default());
        let buf = options.value.clone();
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
            // r_req.spec = Some(prost_types::Any::from(&ByteArray::new(req.spec, req.spec_len)));
            let any = prost_types::Any {
                type_url: "".to_string(), // TODO: fill in the type url
                value: unsafe { std::slice::from_raw_parts(req.spec, req.spec_len) }.to_vec(),
            };
            r_req.spec = Some(any);
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

#[repr(C)]
pub struct SandboxCreateResponse {
    sandbox_id: *const c_char,
    residual: *const c_void,
}

impl SandboxCreateResponse {
    pub fn from_controller(&mut self, req: &sandbox_services::ControllerCreateResponse) {
        self.sandbox_id = CString::new(req.sandbox_id.as_str()).unwrap().into_raw();
    }
}

#[repr(C)]
pub struct SandboxStartRequest {
    sandbox_id: *const c_char,
    sandboxer: *const c_char,
    residual: *const c_void,
}

impl From<&SandboxStartRequest> for sandbox_services::ControllerStartRequest {
    fn from(req: &SandboxStartRequest) -> Self {
        let mut r_req = sandbox_services::ControllerStartRequest::default();
        r_req.sandbox_id = to_string(req.sandbox_id);
        r_req.sandboxer = to_string(req.sandboxer);
        r_req
    }
}

#[repr(C)]
pub struct SandboxStartResponse {
    sandbox_id: *const c_char,
    pid: u32,
    created_at: u64,
    labels: *const MapStringString,
    address: *const c_char,
    version: u32,
    residual: *const c_void,
}

impl SandboxStartResponse {
    pub fn from_controller(&mut self, req: &sandbox_services::ControllerStartResponse) {
        self.sandbox_id = CString::new(req.sandbox_id.as_str()).unwrap().into_raw();
        self.pid = req.pid;
        self.created_at = match &req.created_at {
            Some(timestamp) => prost_timestamp_to_u64(timestamp),
            None => 0
        };
        self.labels = Box::into_raw(Box::new(MapStringString::from(&req.labels)));
        self.address = CString::new(req.address.as_str()).unwrap().into_raw();
        self.version = req.version;
    }
}

#[repr(C)]
pub struct SandboxPlatformRequest {
    sandbox_id: *const c_char,
    sandboxer: *const c_char,
    residual: *const c_void,
}

impl From<&SandboxPlatformRequest> for sandbox_services::ControllerPlatformRequest {
    fn from(req: &SandboxPlatformRequest) -> Self {
        let mut r_req = sandbox_services::ControllerPlatformRequest::default();
        r_req.sandbox_id = to_string(req.sandbox_id);
        r_req.sandboxer = to_string(req.sandboxer);
        r_req
    }
}

#[repr(C)]
pub struct SandboxPlatformResponse {
    os: *const c_char,
    architecture: *const c_char,
    variant: *const c_char,
    residual: *const c_void,
}

impl SandboxPlatformResponse {
    pub fn from_controller(&mut self, rsp: &sandbox_services::ControllerPlatformResponse) {
        rsp.platform.as_ref().map(|platform| {
            self.os = CString::new(platform.os.as_str()).unwrap().into_raw();
            self.architecture = CString::new(platform.architecture.as_str()).unwrap().into_raw();
            self.variant = CString::new(platform.variant.as_str()).unwrap().into_raw();
        });
    }
}

#[repr(C)]
pub struct SandboxStopRequest {
    sandbox_id: *const c_char,
    timeout_secs: u32,
    sandboxer: *const c_char,
    residual: *const c_void,
}

impl From<&SandboxStopRequest> for sandbox_services::ControllerStopRequest {
    fn from(req: &SandboxStopRequest) -> Self {
        let mut r_req = sandbox_services::ControllerStopRequest::default();
        r_req.sandbox_id = to_string(req.sandbox_id);
        r_req.timeout_secs = req.timeout_secs;
        r_req.sandboxer = to_string(req.sandboxer);
        r_req
    }
}

#[repr(C)]
pub struct SandboxWaitRequest {
    sandbox_id: *const c_char,
    sandboxer: *const c_char,
    residual: *const c_void,
}

impl From<&SandboxWaitRequest> for sandbox_services::ControllerWaitRequest {
    fn from(req: &SandboxWaitRequest) -> Self {
        let mut r_req = sandbox_services::ControllerWaitRequest::default();
        r_req.sandbox_id = to_string(req.sandbox_id);
        r_req.sandboxer = to_string(req.sandboxer);
        r_req
    }
}

#[repr(C)]
pub struct SandboxWaitResponse {
    pub sandbox_id: *const c_char,
    exit_status: u32,
    exited_at: u64,
    residual: *const c_void,
}

impl SandboxWaitResponse {
    pub fn from_controller(&mut self, rsp: &sandbox_services::ControllerWaitResponse) {
        self.exit_status = rsp.exit_status;
        self.exited_at = match &rsp.exited_at {
            Some(timestamp) => prost_timestamp_to_u64(timestamp),
            None => 0
        };
    }
}

#[repr(C)]
pub struct SandboxStatusRequest {
    sandbox_id: *const c_char,
    verbose: bool,
    sandboxer: *const c_char,
    residual: *const c_void,
}

impl From<&SandboxStatusRequest> for sandbox_services::ControllerStatusRequest {
    fn from(req: &SandboxStatusRequest) -> Self {
        let mut r_req = sandbox_services::ControllerStatusRequest::default();
        r_req.sandbox_id = to_string(req.sandbox_id);
        r_req.verbose = req.verbose;
        r_req.sandboxer = to_string(req.sandboxer);
        r_req
    }
}

#[repr(C)]
pub struct SandboxStatusResponse {
    sandbox_id: *const c_char,
    pid: u32,
    state: *const c_char,
    info: *const MapStringString,
    created_at: u64,
    exited_at: u64,
    extra: *const u8,
    extra_len: usize,
    address: *const c_char,
    version: u32,
    residual: *const c_void,
}

impl SandboxStatusResponse {
    pub fn from_controller(&mut self, rsp: &sandbox_services::ControllerStatusResponse) {
        self.sandbox_id = CString::new(rsp.sandbox_id.as_str()).unwrap().into_raw();
        self.pid = rsp.pid;
        self.state = CString::new(rsp.state.as_str()).unwrap().into_raw();
        self.info = Box::into_raw(Box::new(MapStringString::from(&rsp.info)));
        self.created_at = match &rsp.created_at {
            Some(timestamp) => prost_timestamp_to_u64(timestamp),
            None => 0
        };
        self.exited_at = match &rsp.exited_at {
            Some(timestamp) => prost_timestamp_to_u64(timestamp),
            None => 0
        };
        rsp.extra.as_ref().map(|extra| {
            let value = extra.value.clone();
            self.extra_len = value.len();
            self.extra = Box::into_raw(Box::new(value.into_boxed_slice())) as *const u8;
        });
        self.address = CString::new(rsp.address.as_str()).unwrap().into_raw();
        self.version = rsp.version;
    }
}

#[repr(C)]
pub struct SandboxShutdownRequest {
    sandbox_id: *const c_char,
    sandboxer: *const c_char,
    residual: *const c_void,
}

impl From<&SandboxShutdownRequest> for sandbox_services::ControllerShutdownRequest {
    fn from(req: &SandboxShutdownRequest) -> Self {
        let mut r_req = sandbox_services::ControllerShutdownRequest::default();
        r_req.sandbox_id = to_string(req.sandbox_id);
        r_req.sandboxer = to_string(req.sandboxer);
        r_req
    }
}

#[repr(C)]
pub struct SandboxMetricsRequest {
    sandbox_id: *const c_char,
    sandboxer: *const c_char,
    residual: *const c_void,
}

impl From<&SandboxMetricsRequest> for sandbox_services::ControllerMetricsRequest {
    fn from(req: &SandboxMetricsRequest) -> Self {
        let mut r_req = sandbox_services::ControllerMetricsRequest::default();
        r_req.sandbox_id = to_string(req.sandbox_id);
        r_req.sandboxer = to_string(req.sandboxer);
        r_req
    }
}

#[repr(C)]
pub struct SandboxMetricsResponse {
    timestamp: u64,
    id: *const c_char,
    data: *const u8,
    data_len: usize,
    residual: *const c_void,
}

impl SandboxMetricsResponse {
    pub fn from_controller(&mut self, rsp: &sandbox_services::ControllerMetricsResponse) {
        let metrics = match rsp.metrics.as_ref() {
            Some(metrics) => metrics,
            None => return
        };
        self.timestamp = match &metrics.timestamp {
            Some(timestamp) => prost_timestamp_to_u64(timestamp),
            None => 0
        };
        self.id = CString::new(metrics.id.as_str()).unwrap().into_raw();
        metrics.data.as_ref().map(|data| {
            let value = data.value.clone();
            self.data_len = value.len();
            self.data = Box::into_raw(Box::new(value.into_boxed_slice())) as *const u8;
        });
    }
}

#[repr(C)]
pub struct SandboxUpdateRequest {
    sandbox_id: *const c_char,
    sandboxer: *const c_char,
    sandbox: *const SandboxSandbox,
    fields: *const *const c_char,
    fields_len: usize,
    residual: *const c_void,
}

impl From<&SandboxUpdateRequest> for sandbox_services::ControllerUpdateRequest {
    fn from(req: &SandboxUpdateRequest) -> Self {
        let mut r_req = sandbox_services::ControllerUpdateRequest::default();
        r_req.sandbox_id = to_string(req.sandbox_id);
        r_req.sandboxer = to_string(req.sandboxer);
        match unsafe { req.sandbox.as_ref() } {
            Some(sandbox) => {
                r_req.sandbox = Some(sandbox::Sandbox::from(&*sandbox));
            }
            None => {
                r_req.sandbox = None;
            }
        }
        r_req.fields = c_char_ptr_ptr_to_vec(req.fields, req.fields_len);
        r_req
    }
}
