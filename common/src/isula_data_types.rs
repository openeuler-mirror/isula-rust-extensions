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
use std::os::raw::c_char;
use prost::Message;

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
pub struct ByteArray {
    pub data: *const u8,
    pub len: usize,
}

impl ByteArray {
    pub fn new(data: *const u8, len: usize) -> Self {
        ByteArray {
            data: data,
            len: len,
        }
    }
}

impl From<&ByteArray> for prost_types::Any {
    fn from(arr: &ByteArray) -> Self {
        if arr.data.is_null() || arr.len == 0 {
            return prost_types::Any::default();
        }
        let encoded = unsafe { std::slice::from_raw_parts(arr.data, arr.len) };
        match prost_types::Any::decode(encoded) {
            Ok(options) => options,
            Err(e) => {
                println!("Failed to decode options, {:?}", e);
                prost_types::Any::default()
            }
        }
    }
}

impl From<&prost_types::Any> for ByteArray {
    fn from(data: &prost_types::Any) -> Self {
        let mut buf = Vec::new();
        match data.encode(& mut buf) {
            Ok(_) => {
                let len = buf.len();
                let buf_data = buf.into_boxed_slice();
                ByteArray {
                    data: Box::into_raw(buf_data) as *const u8,
                    len: len,
                }
            },
            Err(e) => {
                println!("Failed to encode options, {:?}", e);
                ByteArray {
                    data: std::ptr::null(),
                    len: 0,
                }
            }
        }
    }
}

impl Drop for ByteArray {
    fn drop(&mut self) {
        if !self.data.is_null() {
            let _unused = unsafe { Box::from_raw(self.data as *mut u8) };
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

#[repr(C)]
pub struct MapStringBytes {
    key: *const *const c_char,
    value: * const ByteArray,
    len: usize,
}

impl From <&std::collections::HashMap<String, prost_types::Any>> for MapStringBytes {
    fn from(x: &std::collections::HashMap<String, prost_types::Any>) -> Self {
        if x.is_empty() {
            return MapStringBytes {
                key: std::ptr::null(),
                value: std::ptr::null(),
                len: 0,
            }
        }
        let mut keys: Vec<*const c_char> = Vec::new();
        let mut values: Vec<ByteArray> = Vec::new();
        for (key, value) in x.iter() {
            keys.push(CString::new(key.as_str()).unwrap().into_raw());
            values.push(ByteArray::from(value));
        }
        let len = keys.len();
        let keys = keys.into_boxed_slice();
        let values = values.into_boxed_slice();
        let map = MapStringBytes {
            key: Box::into_raw(keys) as *const *const c_char,
            value: Box::into_raw(values) as *const ByteArray,
            len: len,
        };
        map
    }
}

impl From <&MapStringBytes> for std::collections::HashMap<String, prost_types::Any> {
    fn from(x: &MapStringBytes) -> Self {
        let mut map = std::collections::HashMap::new();
        if x.key.is_null() {
            return map;
        }
        let keys: &[*const c_char] = unsafe { std::slice::from_raw_parts(x.key, x.len) };
        let values: &[ByteArray] = unsafe { std::slice::from_raw_parts(x.value, x.len) };
        for i in 0..x.len {
            map.insert(to_string(keys[i]), prost_types::Any::from(&values[i]));
        }
        map
    }
}

impl Drop for MapStringBytes {
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
                let _unused = item;
            }
            let _unused = unsafe { Box::from_raw(self.value as *mut ByteArray) };
        }
    }
}
