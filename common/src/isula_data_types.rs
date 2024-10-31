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
