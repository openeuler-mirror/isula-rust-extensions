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

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void};

pub fn to_string(x: *const c_char) -> String {
    unsafe {
        if x.is_null() {
            "".to_string()
        } else {
            CStr::from_ptr(x).to_str().unwrap_or_default().to_string()
        }
    }
}
pub fn vec_to_double_ptr<T1, T2>(vec: &Vec<T1>) -> (*const *const T2, usize)
where
    T2: for<'a> From<&'a T1>,
{
    let len = vec.len();
    if len == 0 {
        return (std::ptr::null(), 0);
    }

    // Allocate memory for the double pointer
    let double_ptr = vec
        .iter()
        .map(|item| Box::into_raw(Box::new(T2::from(item))) as *const T2)
        .collect::<Vec<*const T2>>()
        .into_boxed_slice();

    // Convert Box<[T]> to *const T
    let double_ptr = Box::into_raw(double_ptr) as *const *const T2;

    (double_ptr, len)
}

pub fn double_ptr_to_vec<T1, T2>(ptr: *const *const T1, len: usize) -> Vec<T2>
where
    T2: for<'a> From<&'a T1>,
    T2: Default,
{
    let mut vec = Vec::new();
    if ptr.is_null() {
        return vec;
    }
    let slice = unsafe { std::slice::from_raw_parts(ptr, len) };
    for item in slice {
        if (*item).is_null() {
            vec.push(T2::default())
        } else {
            vec.push(T2::from(unsafe { item.as_ref() }.unwrap()));
        }
    }
    vec
}

pub fn c_char_ptr_ptr_to_vec(ptr: *const *const c_char, len: usize) -> Vec<String>
{
    let mut vec = Vec::new();
    if ptr.is_null() {
        return vec;
    }
    let slice = unsafe { std::slice::from_raw_parts(ptr, len) };
    for item in slice {
        if item.is_null() {
            vec.push("".to_string());
        } else {
            vec.push(to_string(*item));
        }
    }
    vec
}

pub fn vec_to_c_char_ptr_ptr(vec: &Vec<String>) -> (*const *const c_char, usize)
{
    let len = vec.len();
    if len == 0 {
        return (std::ptr::null(), 0);
    }

    let mut c_char_ptr_vec = Vec::new();
    for item in vec.iter() {
        c_char_ptr_vec.push(CString::new(item.as_str()).unwrap().into_raw());
    }
    let c_char_ptr_vec = c_char_ptr_vec.into_boxed_slice();
    let c_char_ptr = Box::into_raw(c_char_ptr_vec) as *const *const c_char;
    (c_char_ptr, len)
}

#[repr(C)]
pub struct Any {
    pub type_url: *const c_char,
    pub value: *const u8,
    pub len: usize,
    residual: *const c_void,
}

impl From<&Any> for prost_types::Any {
    fn from(any: &Any) -> Self {
        let type_url = to_string(any.type_url);
        let value = if any.value.is_null() || any.len == 0 {
            Vec::new()
        } else {
            unsafe { std::slice::from_raw_parts(any.value, any.len) }.to_vec()
        };
        prost_types::Any {
            type_url: type_url,
            value: value,
        }
    }
}

impl From<&prost_types::Any> for Any {
    fn from(any: &prost_types::Any) -> Self {
        let type_url = CString::new(any.type_url.as_str()).unwrap().into_raw();
        let len = any.value.len();
        let value = if len == 0 {
            std::ptr::null()
        } else {
            let value = any.value.clone();
            let value = value.into_boxed_slice();
            Box::into_raw(value) as *const u8
        };
        Any {
            type_url: type_url,
            value: value,
            len: len,
            residual: std::ptr::null(),
        }
    }
}

const SECOND_TO_NANOS: u64 = 1_000_000_000;
const MAX_NANOS: u64 = 999_999_999;
const MAX_SECONDS: u64 = 253_402_300_799; // 9999-12-31T23:59:59Z
pub fn u64_to_prost_timestamp(data: u64) -> prost_types::Timestamp {
    let mut timestamp = prost_types::Timestamp::default();
    timestamp.seconds = (data / SECOND_TO_NANOS) as i64;
    timestamp.nanos = (data % SECOND_TO_NANOS) as i32;
    timestamp
}

pub fn prost_timestamp_to_u64(data: &prost_types::Timestamp) -> u64 {
    let seconds = if data.seconds < 0 {
        0
    } else if data.seconds as u64 > MAX_SECONDS {
        MAX_SECONDS
    } else {
        data.seconds as u64
    };
    let nanos = if data.nanos < 0 {
        0
    } else if data.nanos as u64 > MAX_NANOS {
        MAX_NANOS
    } else {
        data.nanos as u64
    };
    seconds * SECOND_TO_NANOS + nanos
}

#[repr(C)]
pub struct MapStringString {
    key: *const *const c_char,
    value: *const *const c_char,
    len: usize,
}

impl From <&std::collections::HashMap<String, String>> for MapStringString {
    fn from(x: &std::collections::HashMap<String, String>) -> Self {
        if x.is_empty() {
            return MapStringString {
                key: std::ptr::null(),
                value: std::ptr::null(),
                len: 0,
            }
        }
        let mut keys: Vec<*const c_char> = Vec::new();
        let mut values: Vec<*const c_char> = Vec::new();
        for (key, value) in x.iter() {
            keys.push(CString::new(key.as_str()).unwrap().into_raw());
            values.push(CString::new(value.as_str()).unwrap().into_raw());
        }
        let len = keys.len();
        let keys = keys.into_boxed_slice();
        let values = values.into_boxed_slice();
        let map = MapStringString {
            key: Box::into_raw(keys) as *const *const c_char,
            value: Box::into_raw(values) as *const *const c_char,
            len: len,
        };
        map
    }
}

impl From <&MapStringString> for std::collections::HashMap<String, String> {
    fn from(x: &MapStringString) -> Self {
        let mut map = std::collections::HashMap::new();
        if x.key.is_null() {
            return map;
        }
        let keys: &[*const c_char] = unsafe { std::slice::from_raw_parts(x.key, x.len) };
        let values: &[*const c_char] = unsafe { std::slice::from_raw_parts(x.value, x.len) };
        for i in 0..x.len {
            map.insert(to_string(keys[i]), to_string(values[i]));
        }
        map
    }
}

impl Drop for MapStringString {
    fn drop(&mut self) {
        if !self.key.is_null() {
            let slice = unsafe { std::slice::from_raw_parts(self.key, self.len) };
            for item in slice {
                if !item.is_null() {
                    let _unused = unsafe { CString::from_raw(*item as *mut c_char) };
                }
            }
            let _unused = unsafe { Box::from_raw(self.key as *mut *const c_char) };
        }
        if !self.value.is_null() {
            let slice = unsafe { std::slice::from_raw_parts(self.value, self.len) };
            for item in slice {
                if !item.is_null() {
                    let _unused = unsafe { CString::from_raw(*item as *mut c_char) };
                }
            }
            let _unused = unsafe { Box::from_raw(self.value as *mut *const c_char) };
        }
    }
}
// Extra layer of data structure to adapt to C data structure
#[repr(C)]
pub struct AnyElement {
    pub element: *const Any,
    residual: *const c_void,
}

#[repr(C)]
pub struct MapStringAny {
    key: *const *const c_char,
    value: *const *const AnyElement,
    len: usize,
}

impl From <&std::collections::HashMap<String, prost_types::Any>> for MapStringAny {
    fn from(x: &std::collections::HashMap<String, prost_types::Any>) -> Self {
        if x.is_empty() {
            return MapStringAny {
                key: std::ptr::null(),
                value: std::ptr::null(),
                len: 0,
            }
        }
        let mut keys: Vec<*const c_char> = Vec::new();
        let mut values: Vec<*const AnyElement> = Vec::new();
        for (key, value) in x.iter() {
            keys.push(CString::new(key.as_str()).unwrap().into_raw());
            let any_value = Box::into_raw(Box::new(Any::from(value)));
            let any_elemnt = Box::into_raw(Box::new(AnyElement {
                element: any_value as *const Any,
                residual: std::ptr::null(),
            }));
            values.push(any_elemnt);
        }
        let len = keys.len();
        let keys = keys.into_boxed_slice();
        let values = values.into_boxed_slice();
        let map = MapStringAny {
            key: Box::into_raw(keys) as *const *const c_char,
            value: Box::into_raw(values) as *const *const AnyElement,
            len: len,
        };
        map
    }
}

impl From <&MapStringAny> for std::collections::HashMap<String, prost_types::Any> {
    fn from(x: &MapStringAny) -> Self {
        let mut map = std::collections::HashMap::new();
        if x.key.is_null() {
            return map;
        }
        let keys: &[*const c_char] = unsafe { std::slice::from_raw_parts(x.key, x.len) };
        let values: &[*const AnyElement] = unsafe { std::slice::from_raw_parts(x.value, x.len) };
        for i in 0..x.len {
            match unsafe { values[i].as_ref() } {
                None => continue,
                Some(value) => {
                    match unsafe { value.element.as_ref() } {
                        None => continue,
                        Some(any) => {
                            map.insert(to_string(keys[i]), prost_types::Any::from(any));
                        }
                    }
                }
            }
        }
        map
    }
}
