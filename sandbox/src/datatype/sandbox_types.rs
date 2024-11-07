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
use isula_common::isula_data_types::{Any, MapStringAny, MapStringString};
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
    fn from(mnt: &SandboxMount) -> Self {
        let mut r_mnt = sandbox::Mount::default();
        r_mnt.target = to_string(mnt.destination);
        r_mnt.r#type = to_string(mnt.type_);
        r_mnt.source = to_string(mnt.source);
        r_mnt.options = c_char_ptr_ptr_to_vec(mnt.options, mnt.options_len);
        r_mnt
    }
}

impl From<&sandbox::Mount> for SandboxMount {
    fn from(mnt: &sandbox::Mount) -> Self {
        let (options, options_len) = vec_to_c_char_ptr_ptr(&mnt.options);
        let r_mnt = SandboxMount {
            destination: CString::new(mnt.target.as_str()).unwrap().into_raw(),
            type_: CString::new(mnt.r#type.as_str()).unwrap().into_raw(),
            source: CString::new(mnt.source.as_str()).unwrap().into_raw(),
            options: options,
            options_len: options_len,
            residual: std::ptr::null(),
        };
        r_mnt
    }
}

#[repr(C)]
pub struct SandboxSandboxRuntime {
    name: *const c_char,
    options: *const Any,
    residual: *const c_void,
}

impl From<&SandboxSandboxRuntime> for sandbox::sandbox::Runtime {
    fn from(req: &SandboxSandboxRuntime) -> Self {
        let mut r_req = sandbox::sandbox::Runtime::default();
        r_req.name = to_string(req.name);
        r_req.options = unsafe {req.options.as_ref()}.map(|prost_any| prost_types::Any::from(prost_any));
        r_req
    }
}

impl From<&sandbox::sandbox::Runtime> for SandboxSandboxRuntime {
    fn from(runtime: &sandbox::sandbox::Runtime) -> Self {
        let r_runtime = SandboxSandboxRuntime {
            name: CString::new(runtime.name.as_str()).unwrap().into_raw(),
            options: runtime.options.as_ref()
                .map(|prost_any| Box::into_raw(Box::new(Any::from(prost_any))) as *const Any)
                .unwrap_or(std::ptr::null()),
            residual: std::ptr::null(),
        };
        r_runtime
    }
}

#[repr(C)]
pub struct SandboxSandbox {
    sandbox_id: *const c_char,
    runtime: *const SandboxSandboxRuntime,
    spec: *const Any,
    labels: *const MapStringString,
    created_at: u64,
    updated_at: u64,
    extensions: *const MapStringAny,
    sandboxer: *const c_char,
    residual: *const c_void,
}

impl From<&SandboxSandbox> for sandbox::Sandbox {
    fn from(sandbox: &SandboxSandbox) -> Self {
        let mut r_sandbox = sandbox::Sandbox::default();
        r_sandbox.sandbox_id = to_string(sandbox.sandbox_id);
        r_sandbox.runtime = unsafe { sandbox.runtime.as_ref() }.map(|rt| sandbox::sandbox::Runtime::from(&*rt));
        r_sandbox.spec = unsafe {sandbox.spec.as_ref()}.map(|prost_any| prost_types::Any::from(prost_any));
        r_sandbox.labels = unsafe {sandbox.labels.as_ref()}
            .map(|map| <std::collections::HashMap<String, String>>::from(&*map))
            .unwrap_or(std::collections::HashMap::new());
        r_sandbox.created_at = Some(u64_to_prost_timestamp(sandbox.created_at));
        r_sandbox.updated_at = Some(u64_to_prost_timestamp(sandbox.updated_at));
        r_sandbox.extensions = unsafe {sandbox.extensions.as_ref()}
            .map(|map| <std::collections::HashMap<String, prost_types::Any>>::from(&*map))
            .unwrap_or(std::collections::HashMap::new());
        r_sandbox.sandboxer = to_string(sandbox.sandboxer);
        r_sandbox
    }
}

impl From<&sandbox::Sandbox> for SandboxSandbox {
    fn from(sandbox: &sandbox::Sandbox) -> Self {
        let r_sandbox = SandboxSandbox {
            sandbox_id: CString::new(sandbox.sandbox_id.as_str()).unwrap().into_raw(),
            runtime: sandbox.runtime.as_ref()
                .map(|rt| Box::into_raw(Box::new(SandboxSandboxRuntime::from(rt))) as *const SandboxSandboxRuntime)
                .unwrap_or(std::ptr::null()),
            spec: sandbox.spec.as_ref()
                .map(|prost_any| Box::into_raw(Box::new(Any::from(prost_any))) as *const Any)
                .unwrap_or(std::ptr::null()),
            labels: Box::into_raw(Box::new(MapStringString::from(&sandbox.labels))),
            created_at: sandbox.created_at.as_ref()
                .map(|timestamp| prost_timestamp_to_u64(&timestamp))
                .unwrap_or(0),
            updated_at: sandbox.updated_at.as_ref()
                .map(|timestamp| prost_timestamp_to_u64(&timestamp))
                .unwrap_or(0),
            extensions: Box::into_raw(Box::new(MapStringAny::from(&sandbox.extensions))),
            sandboxer: CString::new(sandbox.sandboxer.as_str()).unwrap().into_raw(),
            residual: std::ptr::null(),
        };
        r_sandbox
    }
}

#[repr(C)]
// order of the data structure is incorrect
pub struct SandboxCreateRequest {
    sandbox_id: *const c_char,
    rootfs: *const *const SandboxMount,
    rootfs_len: usize,
    options: *const Any,
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
        r_req.options = unsafe {req.options.as_ref()}.map(|any| prost_types::Any::from(&*any));
        r_req.netns_path = to_string(req.netns_path);
        r_req.annotations = unsafe {req.annotations.as_ref()}
            .map(|map| <std::collections::HashMap<String, String>>::from(&*map))
            .unwrap_or(std::collections::HashMap::new());
        r_req.sandbox = unsafe { req.sandbox.as_ref() }.map(|sandbox| sandbox::Sandbox::from(&*sandbox));
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
        self.created_at = req.created_at.as_ref()
            .map(|timestamp| prost_timestamp_to_u64(&timestamp))
            .unwrap_or(0);
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
        self.exited_at = rsp.exited_at.as_ref()
            .map(|timestamp| prost_timestamp_to_u64(&timestamp))
            .unwrap_or(0);
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
    extra: *const Any,
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
        self.created_at = rsp.created_at.as_ref()
            .map(|timestamp| prost_timestamp_to_u64(&timestamp))
            .unwrap_or(0);
        self.exited_at = rsp.exited_at.as_ref()
            .map(|timestamp| prost_timestamp_to_u64(&timestamp))
            .unwrap_or(0);
        self.extra = rsp.extra.as_ref()
            .map(|prost_any| {Box::into_raw(Box::new(Any::from(prost_any))) as *const Any})
            .unwrap_or(std::ptr::null());
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
    data: *const Any,
    residual: *const c_void,
}

impl SandboxMetricsResponse {
    pub fn from_controller(&mut self, rsp: &sandbox_services::ControllerMetricsResponse) {
        let metrics = match rsp.metrics.as_ref() {
            Some(metrics) => metrics,
            None => return
        };
        self.timestamp = metrics.timestamp.as_ref()
            .map(|timestamp| prost_timestamp_to_u64(&timestamp))
            .unwrap_or(0);
        self.id = CString::new(metrics.id.as_str()).unwrap().into_raw();
        self.data = metrics.data.as_ref()
            .map(|prost_any| {Box::into_raw(Box::new(Any::from(prost_any))) as *const Any})
            .unwrap_or(std::ptr::null());
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
        r_req.sandbox = unsafe{req.sandbox.as_ref()}.map(|sandbox| sandbox::Sandbox::from(&*sandbox));
        r_req.fields = c_char_ptr_ptr_to_vec(req.fields, req.fields_len);
        r_req
    }
}
