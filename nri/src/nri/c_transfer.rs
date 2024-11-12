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
use std::os::raw::{c_char, c_int, c_void};

use protobuf::{EnumOrUnknown, MessageField};

use crate::protocols::nri::{self, OptionalBool, OptionalFileMode, OptionalInt, OptionalInt64, OptionalString, OptionalUInt32, OptionalUInt64};

use isula_common::isula_data_types::{to_c_char_ptr, to_string};
use isula_common::isula_data_types::vec_to_double_ptr;
use isula_common::isula_data_types::double_ptr_to_vec;
use isula_common::isula_data_types::c_char_ptr_ptr_to_vec;
use isula_common::isula_data_types::vec_to_c_char_ptr_ptr;
use isula_common::isula_data_types::MapStringString;

#[repr(C)]
pub struct NriLinuxMemory {
    limit: *const i64,
    reservation: *const i64,
    swap: *const i64,
    kernel: *const i64,
    kernel_tcp: *const i64,
    swappiness: *const u64,
    disable_oom_killer: *const u8,
    use_hierarchy: *const u8,
    residual: *const c_void,
}

impl From<&NriLinuxMemory> for nri::LinuxMemory {
    fn from(req: &NriLinuxMemory) -> Self {
        let mut r_rq = nri::LinuxMemory::new();
        if !req.limit.is_null() {
            let mut limit = OptionalInt64::new();
            limit.value = unsafe { *req.limit };
            r_rq.limit = MessageField::some(limit);
        }
        if !req.reservation.is_null() {
            let mut reservation = OptionalInt64::new();
            reservation.value = unsafe { *req.reservation };
            r_rq.reservation = MessageField::some(reservation);
        }
        if !req.swap.is_null() {
            let mut swap = OptionalInt64::new();
            swap.value = unsafe { *req.swap };
            r_rq.swap = MessageField::some(swap);
        }
        if !req.kernel.is_null() {
            let mut kernel = OptionalInt64::new();
            kernel.value = unsafe { *req.kernel };
            r_rq.kernel = MessageField::some(kernel);
        }
        if !req.kernel_tcp.is_null() {
            let mut kernel_tcp = OptionalInt64::new();
            kernel_tcp.value = unsafe { *req.kernel_tcp };
            r_rq.kernel_tcp = MessageField::some(kernel_tcp);
        }
        if !req.swappiness.is_null() {
            let mut swappiness = OptionalUInt64::new();
            swappiness.value = unsafe { *req.swappiness };
            r_rq.swappiness = MessageField::some(swappiness);
        }
        if !req.disable_oom_killer.is_null() {
            let mut disable_oom_killer = OptionalBool::new();
            disable_oom_killer.value = unsafe { *req.disable_oom_killer } != 0;
            r_rq.disable_oom_killer = MessageField::some(disable_oom_killer);
        }
        if !req.use_hierarchy.is_null() {
            let mut use_hierarchy = OptionalBool::new();
            use_hierarchy.value = unsafe { *req.use_hierarchy } != 0;
            r_rq.use_hierarchy = MessageField::some(use_hierarchy);
        }
        r_rq

    }
}

impl From<&nri::LinuxMemory> for NriLinuxMemory {
    fn from(req: &nri::LinuxMemory) -> Self {
        let r_req = NriLinuxMemory {
            limit: req.limit.as_ref().map_or(std::ptr::null(), |x| Box::into_raw(Box::new(x.value))),
            reservation: req.reservation.as_ref().map_or(std::ptr::null(), |x| Box::into_raw(Box::new(x.value))),
            swap: req.swap.as_ref().map_or(std::ptr::null(), |x| Box::into_raw(Box::new(x.value))),
            kernel: req.kernel.as_ref().map_or(std::ptr::null(), |x| Box::into_raw(Box::new(x.value))),
            kernel_tcp: req.kernel_tcp.as_ref().map_or(std::ptr::null(), |x| Box::into_raw(Box::new(x.value))),
            swappiness: req.swappiness.as_ref().map_or(std::ptr::null(), |x| Box::into_raw(Box::new(x.value))),
            disable_oom_killer: req.disable_oom_killer.as_ref().map_or(std::ptr::null(), |x| Box::into_raw(Box::new(x.value as u8))),
            use_hierarchy: req.use_hierarchy.as_ref().map_or(std::ptr::null(), |x| Box::into_raw(Box::new(x.value as u8))),
            residual: std::ptr::null(),
        };
        r_req
    }
}

impl Drop for NriLinuxMemory {
    fn drop(&mut self) {
        if !self.limit.is_null() {
            let _unused = unsafe { Box::from_raw(self.limit as *mut i64) };
        }
        if !self.reservation.is_null() {
            let _unused = unsafe { Box::from_raw(self.reservation as *mut i64) };
        }
        if !self.swap.is_null() {
            let _unused = unsafe { Box::from_raw(self.swap as *mut i64) };
        }
        if !self.kernel.is_null() {
            let _unused = unsafe { Box::from_raw(self.kernel as *mut i64) };
        }
        if !self.kernel_tcp.is_null() {
            let _unused = unsafe { Box::from_raw(self.kernel_tcp as *mut i64) };
        }
        if !self.swappiness.is_null() {
            let _unused = unsafe { Box::from_raw(self.swappiness as *mut u64) };
        }
        if !self.disable_oom_killer.is_null() {
            let _unused = unsafe { Box::from_raw(self.disable_oom_killer as *mut u8) };
        }
        if !self.use_hierarchy.is_null() {
            let _unused = unsafe { Box::from_raw(self.use_hierarchy as *mut u8) };
        }
    }

}

#[repr(C)]
pub struct NriLinuxCpu {
    shares: *const u64,
    quota: *const i64,
    period: *const u64,
    realtime_runtime: *const i64,
    realtime_period: *const u64,
    cpus: *const c_char,
    mems: *const c_char,
    residual: *const c_void,
}

impl From<&NriLinuxCpu> for nri::LinuxCPU {
    fn from(req: &NriLinuxCpu) -> Self {
        let mut r_req = nri::LinuxCPU::new();
        if !req.shares.is_null() {
            let mut shares = OptionalUInt64::new();
            shares.value = unsafe { *req.shares };
            r_req.shares = MessageField::some(shares);
        }
        if !req.quota.is_null() {
            let mut quota = OptionalInt64::new();
            quota.value = unsafe { *req.quota };
            r_req.quota = MessageField::some(quota);
        }
        if !req.period.is_null() {
            let mut period = OptionalUInt64::new();
            period.value = unsafe { *req.period };
            r_req.period = MessageField::some(period);
        }
        if !req.realtime_runtime.is_null() {
            let mut realtime_runtime = OptionalInt64::new();
            realtime_runtime.value = unsafe { *req.realtime_runtime };
            r_req.realtime_runtime = MessageField::some(realtime_runtime);
        }
        if !req.realtime_period.is_null() {
            let mut realtime_period = OptionalUInt64::new();
            realtime_period.value = unsafe { *req.realtime_period };
            r_req.realtime_period = MessageField::some(realtime_period);
        }
        r_req.cpus = to_string(req.cpus);
        r_req.mems = to_string(req.mems);
        r_req
    }
}

impl From<&nri::LinuxCPU> for NriLinuxCpu {
    fn from(req: &nri::LinuxCPU) -> Self {
        let r_req = NriLinuxCpu {
            shares: req.shares.as_ref().map_or(std::ptr::null(), |x| Box::into_raw(Box::new(x.value))),
            quota: req.quota.as_ref().map_or(std::ptr::null(), |x| Box::into_raw(Box::new(x.value))),
            period: req.period.as_ref().map_or(std::ptr::null(), |x| Box::into_raw(Box::new(x.value))),
            realtime_runtime: req.realtime_runtime.as_ref().map_or(std::ptr::null(), |x| Box::into_raw(Box::new(x.value))),
            realtime_period: req.realtime_period.as_ref().map_or(std::ptr::null(), |x| Box::into_raw(Box::new(x.value))),
            cpus: to_c_char_ptr(req.cpus.as_str()),
            mems: to_c_char_ptr(req.mems.as_str()),
            residual: std::ptr::null(),
        };
        r_req
    }
}

impl Drop for NriLinuxCpu {
    fn drop(&mut self) {
        if !self.shares.is_null() {
            let _unused = unsafe { Box::from_raw(self.shares as *mut u64) };
        }
        if !self.quota.is_null() {
            let _unused = unsafe { Box::from_raw(self.quota as *mut i64) };
        }
        if !self.period.is_null() {
            let _unused = unsafe { Box::from_raw(self.period as *mut u64) };
        }
        if !self.realtime_runtime.is_null() {
            let _unused = unsafe { Box::from_raw(self.realtime_runtime as *mut i64) };
        }
        if !self.realtime_period.is_null() {
            let _unused = unsafe { Box::from_raw(self.realtime_period as *mut u64) };
        }
        if !self.cpus.is_null() {
            let _unused = unsafe { CString::from_raw(self.cpus as *mut c_char) };
        }
        if !self.mems.is_null() {
            let _unused = unsafe { CString::from_raw(self.mems as *mut c_char) };
        }
    }
}

#[repr(C)]
pub struct NriHugepageLimit {
    page_size: *const c_char,
    limit: u64,
    residual: *const c_void,
}

impl From<&NriHugepageLimit> for nri::HugepageLimit {
    fn from(req: &NriHugepageLimit) -> Self {
        let mut r_req = nri::HugepageLimit::new();
        r_req.page_size = to_string(req.page_size);
        r_req.limit = req.limit;
        r_req
    }
}

impl From<&nri::HugepageLimit> for NriHugepageLimit {
    fn from(req: &nri::HugepageLimit) -> Self {
        let r_req = NriHugepageLimit {
            page_size: to_c_char_ptr(req.page_size.as_str()),
            limit: req.limit,
            residual: std::ptr::null(),
        };
        r_req
    }
}

impl Drop for NriHugepageLimit {
    fn drop(&mut self) {
        if !self.page_size.is_null() {
            let _unused = unsafe { CString::from_raw(self.page_size as *mut c_char) };
        }
    }
}

#[repr(C)]
pub struct NriLinuxDeviceCgroup {
    allow: u8,
    type_: *const c_char,
    major: *const i64,
    minor: *const i64,
    access: *const c_char,
    residual: *const c_void,
}

impl From<&NriLinuxDeviceCgroup> for nri::LinuxDeviceCgroup {
    fn from(req: &NriLinuxDeviceCgroup) -> Self {
        let mut r_req = nri::LinuxDeviceCgroup::new();
        r_req.allow = req.allow != 0;
        r_req.type_ = to_string(req.type_);
        if !req.major.is_null() {
            let mut major = OptionalInt64::new();
            major.value = unsafe { *req.major };
            r_req.major = MessageField::some(major);
        }
        if !req.minor.is_null() {
            let mut minor = OptionalInt64::new();
            minor.value = unsafe { *req.minor };
            r_req.minor = MessageField::some(minor);
        }
        r_req.access = to_string(req.access);
        r_req
    }
}

impl From<&nri::LinuxDeviceCgroup> for NriLinuxDeviceCgroup {
    fn from(req: &nri::LinuxDeviceCgroup) -> Self {
        let r_req = NriLinuxDeviceCgroup {
            allow: req.allow as u8,
            type_: to_c_char_ptr(req.type_.as_str()),
            major: req.major.as_ref().map_or(std::ptr::null(), |x| Box::into_raw(Box::new(x.value))),
            minor: req.minor.as_ref().map_or(std::ptr::null(), |x| Box::into_raw(Box::new(x.value))),
            access: to_c_char_ptr(req.access.as_str()),
            residual: std::ptr::null(),
        };
        r_req
    }
}

impl Drop for NriLinuxDeviceCgroup {
    fn drop(&mut self) {
        if !self.type_.is_null() {
            let _unused = unsafe { CString::from_raw(self.type_ as *mut c_char) };
        }
        if !self.major.is_null() {
            let _unused = unsafe { Box::from_raw(self.major as *mut i64) };
        }
        if !self.minor.is_null() {
            let _unused = unsafe { Box::from_raw(self.minor as *mut i64) };
        }
        if !self.access.is_null() {
            let _unused = unsafe { CString::from_raw(self.access as *mut c_char) };
        }
    }

}
#[repr(C)]
pub struct NriLinuxResources {
    memory: *const NriLinuxMemory,
    cpu: *const NriLinuxCpu,
    hugepage_limits: *const *const NriHugepageLimit,
    hugepage_limits_len: usize,
    blockio_class: *const c_char,
    rdt_class: *const c_char,
    unified: *const MapStringString,
    devices: *const *const NriLinuxDeviceCgroup,
    devices_len: usize,
    residual: *const c_void,
}

impl From<&NriLinuxResources> for nri::LinuxResources {
    fn from(req: &NriLinuxResources) -> Self {
        let mut r_req = nri::LinuxResources::new();
        if !req.memory.is_null() {
            r_req.memory = MessageField::some(nri::LinuxMemory::from(unsafe { req.memory.as_ref() }.unwrap()));
        }
        if !req.cpu.is_null() {
            r_req.cpu = MessageField::some(nri::LinuxCPU::from(unsafe { req.cpu.as_ref() }.unwrap()));
        }
        r_req.hugepage_limits = double_ptr_to_vec(req.hugepage_limits, req.hugepage_limits_len);
        if !req.blockio_class.is_null() {
            let mut blockio_class = OptionalString::new();
            blockio_class.value = to_string(req.blockio_class);
            r_req.blockio_class = MessageField::some(blockio_class);
        }
        if !req.rdt_class.is_null() {
            let mut rdt_class = OptionalString::new();
            rdt_class.value = to_string(req.rdt_class);
            r_req.rdt_class = MessageField::some(rdt_class);
        }
        r_req.unified = match req.unified.is_null() {
            true => std::collections::HashMap::new(),
            false => unsafe { <std::collections::HashMap<String, String>>::from(&*req.unified) },
        };
        r_req.devices = double_ptr_to_vec(req.devices, req.devices_len);
        r_req
    }
}

impl From<&nri::LinuxResources> for NriLinuxResources {
    fn from(req: &nri::LinuxResources) -> Self {
        let (hugepage_limits, hugepage_limits_len) = vec_to_double_ptr(&req.hugepage_limits);
        let (devices, devices_len) = vec_to_double_ptr(&req.devices);
        let r_req = NriLinuxResources {
            memory: req.memory.as_ref().map_or(std::ptr::null(), |x| Box::into_raw(Box::new(NriLinuxMemory::from(x)))),
            cpu: req.cpu.as_ref().map_or(std::ptr::null(), |x| Box::into_raw(Box::new(NriLinuxCpu::from(x)))),
            hugepage_limits: hugepage_limits,
            hugepage_limits_len: hugepage_limits_len,
            blockio_class: req.blockio_class.as_ref().map_or(std::ptr::null(), |x| to_c_char_ptr(x.value.as_str())),
            rdt_class: req.rdt_class.as_ref().map_or(std::ptr::null(), |x| to_c_char_ptr(x.value.as_str())),
            unified: Box::into_raw(Box::new(MapStringString::from(&req.unified))),
            devices: devices,
            devices_len: devices_len,
            residual: std::ptr::null(),
        };
        r_req
    }
}

impl Drop for NriLinuxResources {
    fn drop(&mut self) {
        if !self.memory.is_null() {
            let _unused = unsafe { Box::from_raw(self.memory as *mut NriLinuxMemory) };
        }
        if !self.cpu.is_null() {
            let _unused = unsafe { Box::from_raw(self.cpu as *mut NriLinuxCpu) };
        }
        if !self.hugepage_limits.is_null() {
            let slice = unsafe { std::slice::from_raw_parts(self.hugepage_limits, self.hugepage_limits_len) };
            for item in slice {
                if !item.is_null() {
                    let _unused = unsafe { Box::from_raw(*item as *mut NriHugepageLimit) };
                }
            }
            let _unused = unsafe { Box::from_raw(self.hugepage_limits as *mut *const NriHugepageLimit) };
        }
        if !self.blockio_class.is_null() {
            let _unused = unsafe { CString::from_raw(self.blockio_class as *mut c_char) };
        }
        if !self.rdt_class.is_null() {
            let _unused = unsafe { CString::from_raw(self.rdt_class as *mut c_char) };
        }
        if !self.unified.is_null() {
            let _unused = unsafe { Box::from_raw(self.unified as *mut MapStringString) };
        }
        if !self.devices.is_null() {
            let slice = unsafe { std::slice::from_raw_parts(self.devices, self.devices_len) };
            for item in slice {
                if !item.is_null() {
                    let _unused = unsafe { Box::from_raw(*item as *mut NriLinuxDeviceCgroup) };
                }
            }
            let _unused = unsafe { Box::from_raw(self.devices as *mut *const NriLinuxDeviceCgroup) };
        }
    }
}

#[repr(C)]
pub struct NriLinuxNamespace {
    type_: *const c_char,
    path: *const c_char,
    residual: *const c_void,
}

impl From<&NriLinuxNamespace> for nri::LinuxNamespace {
    fn from(req: &NriLinuxNamespace) -> Self {
        let mut r_req = nri::LinuxNamespace::new();
        r_req.type_ = to_string(req.type_);
        r_req.path = to_string(req.path);
        r_req
    }
}

#[repr(C)]
pub struct NriLinuxPodSandbox {
    pod_overhead: *const NriLinuxResources,
    pod_resources: *const NriLinuxResources,
    cgroup_parent: *const c_char,
    cgroups_path: *const c_char,
    namespaces: *const *const NriLinuxNamespace,
    namespaces_len: usize,
    resources: *const NriLinuxResources,
    residual: *const c_void,
}

impl From<&NriLinuxPodSandbox> for nri::LinuxPodSandbox {
    fn from(req: &NriLinuxPodSandbox) -> Self {
        let mut r_req = nri::LinuxPodSandbox::new();
        if !req.pod_overhead.is_null() {
            r_req.pod_overhead = MessageField::some(nri::LinuxResources::from(unsafe { req.pod_overhead.as_ref() }.unwrap()));
        }
        if !req.pod_resources.is_null() {
            r_req.pod_resources = MessageField::some(nri::LinuxResources::from(unsafe { req.pod_resources.as_ref() }.unwrap()));
        }
        r_req.cgroup_parent = to_string(req.cgroup_parent);
        r_req.cgroups_path = to_string(req.cgroups_path);
        r_req.namespaces = double_ptr_to_vec(req.namespaces, req.namespaces_len);
        if !req.resources.is_null() {
            r_req.resources = MessageField::some(nri::LinuxResources::from(unsafe { req.resources.as_ref() }.unwrap()));
        }
        r_req
    }
}

#[repr(C)]
pub struct NriPodSandbox {
    id: *const c_char,
    name: *const c_char,
    uid: *const c_char,
    namespace: *const c_char,
    labels: *const MapStringString,
    annotations: *const MapStringString,
    runtime_handler: *const c_char,
    linux: *const NriLinuxPodSandbox,
    pid: u32,
    residual: *const c_void,
}

impl From<&NriPodSandbox> for nri::PodSandbox {
    fn from(req: &NriPodSandbox) -> Self {
        let mut r_req = nri::PodSandbox::new();
        r_req.id = to_string(req.id);
        r_req.name = to_string(req.name);
        r_req.uid = to_string(req.uid);
        r_req.namespace = to_string(req.namespace);
        r_req.labels = match req.labels.is_null() {
            true => std::collections::HashMap::new(),
            false => unsafe { <std::collections::HashMap<String, String>>::from(&*req.labels) },
        };
        r_req.annotations = match req.annotations.is_null() {
            true => std::collections::HashMap::new(),
            false => unsafe { <std::collections::HashMap<String, String>>::from(&*req.annotations) },
        };
        r_req.runtime_handler =to_string(req.runtime_handler);
        if !req.linux.is_null() {
            r_req.linux = MessageField::some(nri::LinuxPodSandbox::from(unsafe { req.linux.as_ref() }.unwrap()));
        }
        r_req.pid = req.pid;
        r_req
    }
}

#[repr(C)]
pub struct NriMount {
    destination: *const c_char,
    type_: *const c_char,
    source: *const c_char,
    options: *const *const c_char,
    options_len: usize,
    residual: *const c_void,
}

impl From<&NriMount> for nri::Mount {
    fn from(req: &NriMount) -> Self {
        let mut r_req = nri::Mount::new();
        r_req.destination = to_string(req.destination);
        r_req.type_ = to_string(req.type_);
        r_req.source = to_string(req.source);
        r_req.options = c_char_ptr_ptr_to_vec(req.options, req.options_len);
        r_req
    }
}

impl From<&nri::Mount> for NriMount {
    fn from(req: &nri::Mount) -> Self {
        let (options, options_len) = vec_to_c_char_ptr_ptr(&req.options);
        let r_req = NriMount {
            destination: to_c_char_ptr(req.destination.as_str()),
            type_: to_c_char_ptr(req.type_.as_str()),
            source: to_c_char_ptr(req.source.as_str()),
            options: options,
            options_len: options_len,
            residual: std::ptr::null(),
        };
        r_req
    }
}
#[repr(C)]
pub struct NriHook {
    path: *const c_char,
    args: *const *const c_char,
    args_len: usize,
    env: *const *const c_char,
    env_len: usize,
    timeout: *const i64,
    residual: *const c_void,
}

impl From<&NriHook> for nri::Hook {
    fn from(req: &NriHook) -> Self {
        let mut r_req = nri::Hook::new();
        r_req.path = to_string(req.path);
        r_req.args = c_char_ptr_ptr_to_vec(req.args, req.args_len);
        r_req.env = c_char_ptr_ptr_to_vec(req.env, req.env_len);
        if !req.timeout.is_null() {
            let mut timeout = OptionalInt::new();
            timeout.value = unsafe { *req.timeout };
            r_req.timeout = MessageField::some(timeout);
        }
        r_req
    }
}

impl From<&nri::Hook> for NriHook {
    fn from(req: &nri::Hook) -> Self {
        let (args, args_len) = vec_to_c_char_ptr_ptr(&req.args);
        let (env, env_len) = vec_to_c_char_ptr_ptr(&req.env);
        let r_req = NriHook {
            path: to_c_char_ptr(req.path.as_str()),
            args: args,
            args_len: args_len,
            env: env,
            env_len: env_len,
            timeout: req.timeout.as_ref().map_or(std::ptr::null(), |x| Box::into_raw(Box::new(x.value))),
            residual: std::ptr::null(),
        };
        r_req
    }
}

#[repr(C)]
pub struct NriHooks {
    prestart: *const *const NriHook,
    prestart_len: usize,
    create_runtime: *const *const NriHook,
    create_runtime_len: usize,
    create_container: *const *const NriHook,
    create_container_len: usize,
    start_container: *const *const NriHook,
    start_container_len: usize,
    poststart: *const *const NriHook,
    poststart_len: usize,
    poststop: *const *const NriHook,
    poststop_len: usize,
    residual: *const c_void,
}

impl From<&NriHooks> for nri::Hooks {
    fn from(req: &NriHooks) -> Self {
        let mut r_req = nri::Hooks::new();
        r_req.prestart = double_ptr_to_vec(req.prestart, req.prestart_len);
        r_req.create_runtime = double_ptr_to_vec(req.create_runtime, req.create_runtime_len);
        r_req.create_container = double_ptr_to_vec(req.create_container, req.create_container_len);
        r_req.start_container = double_ptr_to_vec(req.start_container, req.start_container_len);
        r_req.poststart = double_ptr_to_vec(req.poststart, req.poststart_len);
        r_req.poststop = double_ptr_to_vec(req.poststop, req.poststop_len);
        r_req
    }
}

impl From<&nri::Hooks> for NriHooks {
    fn from(req: &nri::Hooks) -> Self {
        let (prestart, prestart_len) = vec_to_double_ptr(&req.prestart);
        let (create_runtime, create_runtime_len) = vec_to_double_ptr(&req.create_runtime);
        let (create_container, create_container_len) = vec_to_double_ptr(&req.create_container);
        let (start_container, start_container_len) = vec_to_double_ptr(&req.start_container);
        let (poststart, poststart_len) = vec_to_double_ptr(&req.poststart);
        let (poststop, poststop_len) = vec_to_double_ptr(&req.poststop);
        let r_req = NriHooks {
            prestart: prestart,
            prestart_len: prestart_len,
            create_runtime: create_runtime,
            create_runtime_len: create_runtime_len,
            create_container: create_container,
            create_container_len: create_container_len,
            start_container: start_container,
            start_container_len: start_container_len,
            poststart: poststart,
            poststart_len: poststart_len,
            poststop: poststop,
            poststop_len: poststop_len,
            residual: std::ptr::null(),
        };
        r_req
    }

}
#[repr(C)]
pub struct NriPosixRlimit {
    type_: *const c_char,
    hard: u64,
    soft: u64,
    residual: *const c_void,
}

impl From<&NriPosixRlimit> for nri::POSIXRlimit {
    fn from(req: &NriPosixRlimit) -> Self {
        let mut r_req = nri::POSIXRlimit::new();
        r_req.type_ = to_string(req.type_);
        r_req.hard = req.hard;
        r_req.soft = req.soft;
        r_req
    }
}

impl From<&nri::POSIXRlimit> for NriPosixRlimit {
    fn from(req: &nri::POSIXRlimit) -> Self {
        let r_req = NriPosixRlimit {
            type_: to_c_char_ptr(req.type_.as_str()),
            hard: req.hard,
            soft: req.soft,
            residual: std::ptr::null(),
        };
        r_req
    }
}

#[repr(C)]
pub struct NriLinuxContainer {
    namespaces: *const *const NriLinuxNamespace,
    namespaces_len: usize,
    devices: *const *const NriLinuxDevice,
    devices_len: usize,
    resources: *const NriLinuxResources,
    oom_score_adj: *const i64,
    cgroups_path: *const c_char,
    residual: *const c_void,
}

impl From<&NriLinuxContainer> for nri::LinuxContainer {
    fn from(req: &NriLinuxContainer) -> Self {
        let mut r_req = nri::LinuxContainer::new();
        r_req.namespaces = double_ptr_to_vec(req.namespaces, req.namespaces_len);
        r_req.devices = double_ptr_to_vec(req.devices, req.devices_len);
        if !req.resources.is_null() {
            r_req.resources = MessageField::some(nri::LinuxResources::from(unsafe { req.resources.as_ref() }.unwrap()));
        }
        if !req.oom_score_adj.is_null() {
            let mut oom_score_adj = OptionalInt::new();
            oom_score_adj.value = unsafe { *req.oom_score_adj };
            r_req.oom_score_adj = MessageField::some(oom_score_adj);
        }
        r_req.cgroups_path = to_string(req.cgroups_path);
        r_req
    }
}

#[repr(C)]
pub struct NriContainer {
    id: *const c_char,
    pod_sandbox_id: *const c_char,
    name: *const c_char,
    state: i32,
    labels: *const MapStringString,
    annotations: *const MapStringString,
    args: *const *const c_char,
    args_len: usize,
    env: *const *const c_char,
    env_len: usize,
    mounts: *const *const NriMount,
    mounts_len: usize,
    hooks: *const NriHooks,
    linux: *const NriLinuxContainer,
    pid: u32,
    rlimits: *const *const NriPosixRlimit,
    rlimits_len: usize,
    residual: *const c_void,
}

impl From<&NriContainer> for nri::Container {
    fn from(req: &NriContainer) -> Self {
        let mut r_req = nri::Container::new();
        r_req.id = to_string(req.id);
        r_req.pod_sandbox_id = to_string(req.pod_sandbox_id);
        r_req.name = to_string(req.name);
        r_req.state = EnumOrUnknown::from_i32(req.state);
        r_req.labels = match req.labels.is_null() {
            true => std::collections::HashMap::new(),
            false => unsafe { <std::collections::HashMap<String, String>>::from(&*req.labels) },
        };
        r_req.annotations = match req.annotations.is_null() {
            true => std::collections::HashMap::new(),
            false => unsafe { <std::collections::HashMap<String, String>>::from(&*req.annotations) },
        };
        r_req.args = c_char_ptr_ptr_to_vec(req.args, req.args_len);
        r_req.env = c_char_ptr_ptr_to_vec(req.env, req.env_len);
        r_req.mounts = double_ptr_to_vec(req.mounts, req.mounts_len);
        if !req.hooks.is_null() {
            r_req.hooks = MessageField::some(nri::Hooks::from(unsafe { req.hooks.as_ref() }.unwrap()));
        }
        if !req.linux.is_null() {
            r_req.linux = MessageField::some(nri::LinuxContainer::from(unsafe { req.linux.as_ref() }.unwrap()));
        }
        r_req.pid = req.pid;
        r_req.rlimits = double_ptr_to_vec(req.rlimits, req.rlimits_len);
        r_req
    }
}

#[repr(C)]
pub struct NriLinuxContainerUpdate {
    resources: *const NriLinuxResources,
    residual: *const c_void,
}

impl From<&NriLinuxContainerUpdate> for nri::LinuxContainerUpdate {
    fn from(req: &NriLinuxContainerUpdate) -> Self {
        let mut r_req = nri::LinuxContainerUpdate::new();
        if !req.resources.is_null() {
            r_req.resources = MessageField::some(nri::LinuxResources::from(unsafe { req.resources.as_ref() }.unwrap()));
        }
        r_req
    }
}

impl From<&nri::LinuxContainerUpdate> for NriLinuxContainerUpdate {
    fn from(req: &nri::LinuxContainerUpdate) -> Self {
        let r_req = NriLinuxContainerUpdate {
            resources: req.resources.as_ref().map_or(std::ptr::null(), |x| Box::into_raw(Box::new(NriLinuxResources::from(x)))),
            residual: std::ptr::null(),
        };
        r_req
    }
}

impl Drop for NriLinuxContainerUpdate {
    fn drop(&mut self) {
        if !self.resources.is_null() {
            let _unused = unsafe { Box::from_raw(self.resources as *mut NriLinuxResources) };
        }
    }
}

#[repr(C)]
pub struct NriContainerUpdate {
    container_id: *const c_char,
    linux: *const NriLinuxContainerUpdate,
    ignore_failure: u8,
    residual: *const c_void,
}

impl From<&nri::ContainerUpdate> for NriContainerUpdate {
    fn from(req: &nri::ContainerUpdate) -> Self {
        let r_req = NriContainerUpdate {
            container_id: to_c_char_ptr(req.container_id.as_str()),
            linux: req.linux.as_ref().map_or(std::ptr::null(), |x| Box::into_raw(Box::new(NriLinuxContainerUpdate::from(x)))),
            ignore_failure: req.ignore_failure as u8,
            residual: std::ptr::null(),
        };
        r_req
    }
}

impl From<&NriContainerUpdate> for nri::ContainerUpdate {
    fn from(req: &NriContainerUpdate) -> Self {
        let mut r_req = nri::ContainerUpdate::new();
        r_req.container_id = to_string(req.container_id);
        if !req.linux.is_null() {
            r_req.linux = MessageField::some(nri::LinuxContainerUpdate::from(unsafe { req.linux.as_ref() }.unwrap()));
        }
        r_req.ignore_failure = (req.ignore_failure != 0) as bool;
        r_req
    }
}

impl Drop for NriContainerUpdate {
    fn drop(&mut self) {
        if !self.container_id.is_null() {
            let _unused = unsafe { CString::from_raw(self.container_id as *mut c_char) };
        }
        if !self.linux.is_null() {
            let _unused = unsafe { Box::from_raw(self.linux as *mut NriLinuxContainerUpdate) };
        }
    }
}
#[repr(C)]
pub struct NriKeyValue {
    key: *const c_char,
    value: *const c_char,
    residual: *const c_void,
}

impl From<&nri::KeyValue> for NriKeyValue {
    fn from(req: &nri::KeyValue) -> Self {
        let r_req = NriKeyValue {
            key: to_c_char_ptr(req.key.as_str()),
            value: to_c_char_ptr(req.value.as_str()),
            residual: std::ptr::null(),
        };
        r_req
    }
}

#[repr(C)]
pub struct NriLinuxDevice {
    path: *const c_char,
    type_: *const c_char,
    major: i64,
    minor: i64,
    file_mode: *const u32,
    uid: *const u32,
    gid: *const u32,
    residual: *const c_void,
}

impl From<&NriLinuxDevice> for nri::LinuxDevice {
    fn from(req: &NriLinuxDevice) -> Self {
        let mut r_req = nri::LinuxDevice::new();
        r_req.path = to_string(req.path);
        r_req.type_ = to_string(req.type_);
        r_req.major = req.major;
        r_req.minor = req.minor;
        if !req.file_mode.is_null() {
            let mut file_mode = OptionalFileMode::new();
            file_mode.value = unsafe { *req.file_mode };
            r_req.file_mode = MessageField::some(file_mode);
        }
        if !req.uid.is_null() {
            let mut uid = OptionalUInt32::new();
            uid.value = unsafe { *req.uid};
            r_req.uid = MessageField::some(uid);
        }
        if !req.gid.is_null() {
            let mut gid = OptionalUInt32::new();
            gid.value = unsafe { *req.gid };
            r_req.gid = MessageField::some(gid);
        }
        r_req
    }
}

impl From<&nri::LinuxDevice> for NriLinuxDevice {
    fn from(req: &nri::LinuxDevice) -> Self {
        let r_req = NriLinuxDevice {
            path: to_c_char_ptr(req.path.as_str()),
            type_: to_c_char_ptr(req.type_.as_str()),
            major: req.major,
            minor: req.minor,
            file_mode: req.file_mode.as_ref().map_or(std::ptr::null(), |x| Box::into_raw(Box::new(x.value))),
            uid: req.uid.as_ref().map_or(std::ptr::null(), |x| Box::into_raw(Box::new(x.value))),
            gid: req.gid.as_ref().map_or(std::ptr::null(), |x| Box::into_raw(Box::new(x.value))),
            residual: std::ptr::null(),
        };
        r_req
    }
}

#[repr(C)]
pub struct NriLinuxContainerAdjustment {
    devices: *const *const NriLinuxDevice,
    devices_len: usize,
    resources: *const NriLinuxResources,
    cgroups_path: *const c_char,
    residual: *const c_void,
}

impl From<&nri::LinuxContainerAdjustment> for NriLinuxContainerAdjustment {
    fn from(req: &nri::LinuxContainerAdjustment) -> Self {
        let (devices, devices_len) = vec_to_double_ptr(&req.devices);
        let r_req = NriLinuxContainerAdjustment {
            devices: devices,
            devices_len: devices_len,
            resources: req.resources.as_ref().map_or(std::ptr::null(), |x| Box::into_raw(Box::new(NriLinuxResources::from(x)))),
            cgroups_path: to_c_char_ptr(req.cgroups_path.as_str()),
            residual: std::ptr::null(),
        };
        r_req
    }
}

#[repr(C)]
pub struct NriContainerAdjustment {
    annotations: *const MapStringString,
    mounts: *const *const NriMount,
    mounts_len: usize,
    env: *const *const NriKeyValue,
    env_len: usize,
    hooks: *const NriHooks,
    linux: *const NriLinuxContainerAdjustment,
    rlimits: *const *const NriPosixRlimit,
    rlimits_len: usize,
    residual: *const c_void,
}

impl From<&nri::ContainerAdjustment> for NriContainerAdjustment {
    fn from(req: &nri::ContainerAdjustment) -> Self {
        let (mounts, mounts_len) = vec_to_double_ptr(&req.mounts);
        let (env, env_len) = vec_to_double_ptr(&req.env);
        let (rlimits, rlimits_len) = vec_to_double_ptr(&req.rlimits);
        let r_req = NriContainerAdjustment {
            annotations: Box::into_raw(Box::new(MapStringString::from(&req.annotations))),
            mounts: mounts,
            mounts_len: mounts_len,
            env: env,
            env_len: env_len,
            hooks: req.hooks.as_ref().map_or(std::ptr::null(), |x| Box::into_raw(Box::new(NriHooks::from(x)))),
            linux: req.linux.as_ref().map_or(std::ptr::null(), |x| Box::into_raw(Box::new(NriLinuxContainerAdjustment::from(x)))),
            rlimits: rlimits,
            rlimits_len: rlimits_len,
            residual: std::ptr::null(),
        };
        r_req
    }
}

#[repr(C)]
pub struct NriContainerEviction {
    container_id: *const c_char,
    reason: *const c_char,
    residual: *const c_void,
}

impl From<&nri::ContainerEviction> for NriContainerEviction {
    fn from(req: &nri::ContainerEviction) -> Self {
        let r_req = NriContainerEviction {
            container_id: to_c_char_ptr(req.container_id.as_str()),
            reason: to_c_char_ptr(req.reason.as_str()),
            residual: std::ptr::null(),
        };
        r_req
    }
}

impl Drop for NriContainerEviction {
    fn drop(&mut self) {
        if !self.container_id.is_null() {
            let _unused = unsafe { CString::from_raw(self.container_id as *mut c_char) };
        }
        if !self.reason.is_null() {
            let _unused = unsafe { CString::from_raw(self.reason as *mut c_char) };
        }
    }
}

#[repr(C)]
pub struct NriRegisterPluginRequest {
    plugin_name: *const c_char,
    plugin_idx: *const c_char,
    residual: *const c_void,
}

impl From<&nri::RegisterPluginRequest> for NriRegisterPluginRequest {
    fn from(req: &nri::RegisterPluginRequest) -> Self {
        let r_req = NriRegisterPluginRequest {
            plugin_name: to_c_char_ptr(req.plugin_name.as_str()),
            plugin_idx: to_c_char_ptr(req.plugin_idx.as_str()),
            residual: std::ptr::null(),
        };
        r_req
    }
}

impl Drop for NriRegisterPluginRequest {
    fn drop(&mut self) {
        if !self.plugin_name.is_null() {
            let _unused = unsafe { CString::from_raw(self.plugin_name as *mut c_char) };
        }
        if !self.plugin_idx.is_null() {
            let _unused = unsafe { CString::from_raw(self.plugin_idx as *mut c_char) };
        }
    }
}

#[repr(C)]
pub struct NriUpdateContainersRequest {
    container_updates: *const *const NriContainerUpdate,
    container_updates_len: usize,
    evict: *const *const NriContainerEviction,
    evict_len: usize,
    residual: *const c_void,
}

impl From<&nri::UpdateContainersRequest> for NriUpdateContainersRequest {
    fn from(req: &nri::UpdateContainersRequest) -> Self {
        let (update, update_len) = vec_to_double_ptr(&req.update);
        let (evict, evict_len) = vec_to_double_ptr(&req.evict);
        let r_req = NriUpdateContainersRequest {
            container_updates: update,
            container_updates_len: update_len,
            evict: evict,
            evict_len: evict_len,
            residual: std::ptr::null(),
        };
        r_req
    }
}

impl Drop for NriUpdateContainersRequest {
    fn drop(&mut self) {
        if !self.container_updates.is_null() {
            let update = unsafe { std::slice::from_raw_parts(self.container_updates, self.container_updates_len) };
            for i in 0..self.container_updates_len {
                if !update[i].is_null() {
                    let _unused = unsafe { Box::from_raw(update[i] as *mut NriContainerUpdate) };
                }
            }
            let _unused = unsafe { Box::from_raw(self.container_updates as *mut *const NriContainerUpdate) };
        }
        if !self.evict.is_null() {
            let evict = unsafe { std::slice::from_raw_parts(self.evict, self.evict_len) };
            for i in 0..self.evict_len {
                if !evict[i].is_null() {
                    let _unused = unsafe { Box::from_raw(evict[i] as *mut NriContainerEviction) };
                }
            }
            let _unused = unsafe { Box::from_raw(self.evict as *mut *const NriContainerEviction) };
        }
    }
}

#[repr(C)]
pub struct NriUpdateContainersResponse {
    failed: *const *const NriContainerUpdate,
    failed_len: usize,
    residual: *const c_void,
}

impl From<&NriUpdateContainersResponse> for nri::UpdateContainersResponse {
    fn from(resp: &NriUpdateContainersResponse) -> Self {
        let mut r_resp = nri::UpdateContainersResponse::new();
        r_resp.failed = double_ptr_to_vec(resp.failed, resp.failed_len);
        r_resp
    }
}

impl Drop for NriUpdateContainersResponse {
    fn drop(&mut self) {
        if !self.failed.is_null() {
            let failed = unsafe { std::slice::from_raw_parts(self.failed, self.failed_len) };
            for i in 0..self.failed_len {
                if !failed[i].is_null() {
                    let _unused = unsafe { Box::from_raw(failed[i] as *mut NriContainerUpdate) };
                }
            }
            let _unused = unsafe { Box::from_raw(self.failed as *mut *const NriContainerUpdate) };
        }
    }
}

#[repr(C)]
pub struct NriConfigureRequest {
    config: *const c_char,
    runtime_name: *const c_char,
    runtime_version: *const c_char,
    residual: *const c_void,
}

impl From<&NriConfigureRequest> for nri::ConfigureRequest {
    fn from(req: &NriConfigureRequest) -> Self {
        let mut r_resp = nri::ConfigureRequest::new();
        r_resp.config = to_string(req.config);
        r_resp.runtime_name = to_string(req.runtime_name);
        r_resp.runtime_version = to_string(req.runtime_version);
        r_resp
    }
}

#[repr(C)]
pub struct NriConfigureResponse {
    events: i32,
    residual: *const c_void,
}

impl From<&nri::ConfigureResponse> for NriConfigureResponse {
    fn from(resp: &nri::ConfigureResponse) -> Self {
        let r_resp = NriConfigureResponse {
            events: resp.events,
            residual: std::ptr::null(),
        };
        r_resp
    }
}

#[repr(C)]
pub struct NriSynchronizeRequest {
    pods: *const *const NriPodSandbox,
    pods_len: usize,
    containers: *const *const NriContainer,
    containers_len: usize,
    residual: *const c_void,
}

impl From<&NriSynchronizeRequest> for nri::SynchronizeRequest {
    fn from(req: &NriSynchronizeRequest) -> Self {
        let mut r_req = nri::SynchronizeRequest::new();
        r_req.pods = double_ptr_to_vec(req.pods, req.pods_len);
        r_req.containers = double_ptr_to_vec(req.containers, req.containers_len);
        r_req
    }
}

#[repr(C)]
pub struct NriSynchronizeResponse {
    update: *const *const NriContainerUpdate,
    update_len: usize,
    residual: *const c_void,
}

impl From<&nri::SynchronizeResponse> for NriSynchronizeResponse {
    fn from(resp: &nri::SynchronizeResponse) -> Self {
        let (update, update_len) = vec_to_double_ptr(&resp.update);
        let r_resp = NriSynchronizeResponse {
            update: update,
            update_len: update_len,
            residual: std::ptr::null(),
        };
        r_resp
    }
}

#[repr(C)]
pub struct NriCreateContainerRequest {
    pod: *const NriPodSandbox,
    container: *const NriContainer,
    residual: *const c_void,
}

impl From<&NriCreateContainerRequest> for nri::CreateContainerRequest {
    fn from(req: &NriCreateContainerRequest) -> Self {
        let mut r_req = nri::CreateContainerRequest::new();
        if !req.pod.is_null() {
            r_req.pod = MessageField::some(nri::PodSandbox::from(unsafe { req.pod.as_ref() }.unwrap()));
        }
        if !req.container.is_null() {
            r_req.container = MessageField::some(nri::Container::from(unsafe { req.container.as_ref() }.unwrap()));
        }
        r_req
    }
}

#[repr(C)]
pub struct NriCreateContainerResponse {
    adjust: *const NriContainerAdjustment,
    update: *const *const NriContainerUpdate,
    update_len: usize,
    evict: *const *const NriContainerEviction,
    evict_len: usize,
    residual: *const c_void,
}

impl From<&nri::CreateContainerResponse> for NriCreateContainerResponse {
    fn from(resp: &nri::CreateContainerResponse) -> Self {
        let (update, update_len) = vec_to_double_ptr(&resp.update);
        let (evict, evict_len) = vec_to_double_ptr(&resp.evict);
        let r_resp = NriCreateContainerResponse {
            adjust: Box::into_raw(Box::new(NriContainerAdjustment::from(resp.adjust.as_ref().unwrap()))),
            update: update,
            update_len: update_len,
            evict: evict,
            evict_len: evict_len,
            residual: std::ptr::null(),
        };
        r_resp
    
    }
}

#[repr(C)]
pub struct NriUpdateContainerRequest {
    pod: *const NriPodSandbox,
    container: *const NriContainer,
    linux_resources: *const NriLinuxResources,
    residual: *const c_void,
}

impl From<&NriUpdateContainerRequest> for nri::UpdateContainerRequest {
    fn from(req: &NriUpdateContainerRequest) -> Self {
        let mut r_req = nri::UpdateContainerRequest::new();
        if !req.pod.is_null() {
            r_req.pod = MessageField::some(nri::PodSandbox::from(unsafe { req.pod.as_ref() }.unwrap()));
        }
        if !req.container.is_null() {
            r_req.container = MessageField::some(nri::Container::from(unsafe { req.container.as_ref() }.unwrap()));
        }
        if !req.linux_resources.is_null() {
            r_req.linux_resources = MessageField::some(nri::LinuxResources::from(unsafe { req.linux_resources.as_ref() }.unwrap()));
        }
        r_req
    }
}

#[repr(C)]
pub struct NriUpdateContainerResponse {
    update: *const *const NriContainerUpdate,
    update_len: usize,
    evict: *const *const NriContainerEviction,
    evict_len: usize,
    residual: *const c_void,
}

impl From<&nri::UpdateContainerResponse> for NriUpdateContainerResponse {
    fn from(resp: &nri::UpdateContainerResponse) -> Self {
        let (update, update_len) = vec_to_double_ptr(&resp.update);
        let (evict, evict_len) = vec_to_double_ptr(&resp.evict);
        let r_resp = NriUpdateContainerResponse {
            update: update,
            update_len: update_len,
            evict: evict,
            evict_len: evict_len,
            residual: std::ptr::null(),
        };
        r_resp
    }
}

#[repr(C)]
pub struct NriStopContainerRequest {
    pod: *const NriPodSandbox,
    container: *const NriContainer,
    residual: *const c_void,
}

impl From<&NriStopContainerRequest> for nri::StopContainerRequest {
    fn from(req: &NriStopContainerRequest) -> Self {
        let mut r_req = nri::StopContainerRequest::new();
        if !req.pod.is_null() {
            r_req.pod = MessageField::some(nri::PodSandbox::from(unsafe { req.pod.as_ref() }.unwrap()));
        }
        if !req.container.is_null() {
            r_req.container = MessageField::some(nri::Container::from(unsafe { req.container.as_ref() }.unwrap()));
        }
        r_req
    }
}

#[repr(C)]
pub struct NriStopContainerResponse {
    update: *const *const NriContainerUpdate,
    update_len: usize,
    residual: *const c_void,
}

impl From<&nri::StopContainerResponse> for NriStopContainerResponse {
    fn from(resp: &nri::StopContainerResponse) -> Self {
        let (update, update_len) = vec_to_double_ptr(&resp.update);
        let r_resp = NriStopContainerResponse {
            update: update,
            update_len: update_len,
            residual: std::ptr::null(),
        };
        r_resp
    }
}

#[repr(C)]
pub struct NriStateChangeEvent {
    event: i32,
    pod: *const NriPodSandbox,
    container: *const NriContainer,
    residual: *const c_void,
}

impl From<&NriStateChangeEvent> for nri::StateChangeEvent {
    fn from(req: &NriStateChangeEvent) -> Self {
        let mut r_req = nri::StateChangeEvent::new();
        r_req.event = EnumOrUnknown::from_i32(req.event);
        if !req.pod.is_null() {
            r_req.pod = MessageField::some(nri::PodSandbox::from(unsafe { req.pod.as_ref() }.unwrap()));
        }
        if !req.container.is_null() {
            r_req.container = MessageField::some(nri::Container::from(unsafe { req.container.as_ref() }.unwrap()));
        }
        r_req
    }
}

pub type NriRuntimeRegisterCallback = extern "C" fn(*const c_char, *const NriRegisterPluginRequest) -> c_int;
pub type NriRuntimeUpdateContainersCallback = extern "C" fn(*const c_char, *const NriUpdateContainersRequest, *mut *mut NriUpdateContainersResponse) -> c_int;

#[derive(Clone)]
#[repr(C)]
pub struct NriRuntimeCallbacks {
    pub register_plugin: Option<NriRuntimeRegisterCallback>,
    pub update_containers: Option<NriRuntimeUpdateContainersCallback>,
}

pub type NriExternalConnectCallback = extern "C" fn(c_int) -> c_int;
