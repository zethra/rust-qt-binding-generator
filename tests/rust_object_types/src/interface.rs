/* generated by rust_qt_binding_generator */
#![allow(unknown_lints)]
#![allow(mutex_atomic, needless_pass_by_value)]
use libc::{c_int, c_void, uint8_t, uint16_t};
use std::slice;

use std::sync::{Arc, Mutex};
use std::ptr::null;

use implementation::*;


#[repr(C)]
pub struct COption<T> {
    data: T,
    some: bool,
}

impl<T> From<Option<T>> for COption<T>
where
    T: Default,
{
    fn from(t: Option<T>) -> COption<T> {
        if let Some(v) = t {
            COption {
                data: v,
                some: true,
            }
        } else {
            COption {
                data: T::default(),
                some: false,
            }
        }
    }
}


#[repr(C)]
pub struct QString {
    data: *const uint8_t,
    len: c_int,
}

#[repr(C)]
pub struct QStringIn {
    data: *const uint16_t,
    len: c_int,
}

impl QStringIn {
    fn convert(&self) -> String {
        let data = unsafe { slice::from_raw_parts(self.data, self.len as usize) };
        String::from_utf16_lossy(data)
    }
}

impl<'a> From<&'a str> for QString {
    fn from(string: &'a str) -> QString {
        QString {
            len: string.len() as c_int,
            data: string.as_ptr(),
        }
    }
}

impl<'a> From<&'a String> for QString {
    fn from(string: &'a String) -> QString {
        QString {
            len: string.len() as c_int,
            data: string.as_ptr(),
        }
    }
}


#[repr(C)]
pub struct QByteArray {
    data: *const uint8_t,
    len: c_int,
}

impl QByteArray {
    fn convert(&self) -> Vec<u8> {
        let data = unsafe { slice::from_raw_parts(self.data, self.len as usize) };
        Vec::from(data)
    }
}

impl<'a> From<&'a [u8]> for QByteArray {
    fn from(value: &'a [u8]) -> QByteArray {
        QByteArray {
            len: value.len() as c_int,
            data: value.as_ptr(),
        }
    }
}

pub struct ObjectQObject {}

#[derive(Clone)]
pub struct ObjectEmitter {
    qobject: Arc<Mutex<*const ObjectQObject>>,
    boolean_changed: fn(*const ObjectQObject),
    bytearray_changed: fn(*const ObjectQObject),
    integer_changed: fn(*const ObjectQObject),
    optional_bytearray_changed: fn(*const ObjectQObject),
    optional_string_changed: fn(*const ObjectQObject),
    string_changed: fn(*const ObjectQObject),
    u64_changed: fn(*const ObjectQObject),
    uinteger_changed: fn(*const ObjectQObject),
}

unsafe impl Send for ObjectEmitter {}

impl ObjectEmitter {
    fn clear(&self) {
        *self.qobject.lock().unwrap() = null();
    }
    pub fn boolean_changed(&self) {
        let ptr = *self.qobject.lock().unwrap();
        if !ptr.is_null() {
            (self.boolean_changed)(ptr);
        }
    }
    pub fn bytearray_changed(&self) {
        let ptr = *self.qobject.lock().unwrap();
        if !ptr.is_null() {
            (self.bytearray_changed)(ptr);
        }
    }
    pub fn integer_changed(&self) {
        let ptr = *self.qobject.lock().unwrap();
        if !ptr.is_null() {
            (self.integer_changed)(ptr);
        }
    }
    pub fn optional_bytearray_changed(&self) {
        let ptr = *self.qobject.lock().unwrap();
        if !ptr.is_null() {
            (self.optional_bytearray_changed)(ptr);
        }
    }
    pub fn optional_string_changed(&self) {
        let ptr = *self.qobject.lock().unwrap();
        if !ptr.is_null() {
            (self.optional_string_changed)(ptr);
        }
    }
    pub fn string_changed(&self) {
        let ptr = *self.qobject.lock().unwrap();
        if !ptr.is_null() {
            (self.string_changed)(ptr);
        }
    }
    pub fn u64_changed(&self) {
        let ptr = *self.qobject.lock().unwrap();
        if !ptr.is_null() {
            (self.u64_changed)(ptr);
        }
    }
    pub fn uinteger_changed(&self) {
        let ptr = *self.qobject.lock().unwrap();
        if !ptr.is_null() {
            (self.uinteger_changed)(ptr);
        }
    }
}

pub trait ObjectTrait {
    fn create(emit: ObjectEmitter) -> Self;
    fn emit(&self) -> &ObjectEmitter;
    fn boolean(&self) -> bool;
    fn set_boolean(&mut self, value: bool);
    fn bytearray(&self) -> &[u8];
    fn set_bytearray(&mut self, value: Vec<u8>);
    fn integer(&self) -> i32;
    fn set_integer(&mut self, value: i32);
    fn optional_bytearray(&self) -> Option<&[u8]>;
    fn set_optional_bytearray(&mut self, value: Option<Vec<u8>>);
    fn optional_string(&self) -> Option<&str>;
    fn set_optional_string(&mut self, value: Option<String>);
    fn string(&self) -> &str;
    fn set_string(&mut self, value: String);
    fn u64(&self) -> u64;
    fn set_u64(&mut self, value: u64);
    fn uinteger(&self) -> u32;
    fn set_uinteger(&mut self, value: u32);
}

#[no_mangle]
pub extern "C" fn object_new(
    object: *mut ObjectQObject,
    boolean_changed: fn(*const ObjectQObject),
    bytearray_changed: fn(*const ObjectQObject),
    integer_changed: fn(*const ObjectQObject),
    optional_bytearray_changed: fn(*const ObjectQObject),
    optional_string_changed: fn(*const ObjectQObject),
    string_changed: fn(*const ObjectQObject),
    u64_changed: fn(*const ObjectQObject),
    uinteger_changed: fn(*const ObjectQObject),
) -> *mut Object {
    let object_emit = ObjectEmitter {
        qobject: Arc::new(Mutex::new(object)),
        boolean_changed: boolean_changed,
        bytearray_changed: bytearray_changed,
        integer_changed: integer_changed,
        optional_bytearray_changed: optional_bytearray_changed,
        optional_string_changed: optional_string_changed,
        string_changed: string_changed,
        u64_changed: u64_changed,
        uinteger_changed: uinteger_changed,
    };
    let d_object = Object::create(object_emit);
    Box::into_raw(Box::new(d_object))
}

#[no_mangle]
pub unsafe extern "C" fn object_free(ptr: *mut Object) {
    Box::from_raw(ptr).emit().clear();
}

#[no_mangle]
pub unsafe extern "C" fn object_boolean_get(ptr: *const Object) -> bool {
    (&*ptr).boolean()
}

#[no_mangle]
pub unsafe extern "C" fn object_boolean_set(ptr: *mut Object, v: bool) {
    (&mut *ptr).set_boolean(v);
}

#[no_mangle]
pub unsafe extern "C" fn object_bytearray_get(
    ptr: *const Object,
    p: *mut c_void,
    set: fn(*mut c_void, QByteArray),
) {
    let data = (&*ptr).bytearray();
    set(p, data.into());
}

#[no_mangle]
pub unsafe extern "C" fn object_bytearray_set(ptr: *mut Object, v: QByteArray) {
    (&mut *ptr).set_bytearray(v.convert());
}

#[no_mangle]
pub unsafe extern "C" fn object_integer_get(ptr: *const Object) -> i32 {
    (&*ptr).integer()
}

#[no_mangle]
pub unsafe extern "C" fn object_integer_set(ptr: *mut Object, v: i32) {
    (&mut *ptr).set_integer(v);
}

#[no_mangle]
pub unsafe extern "C" fn object_optional_bytearray_get(
    ptr: *const Object,
    p: *mut c_void,
    set: fn(*mut c_void, QByteArray),
) {
    let data = (&*ptr).optional_bytearray();
    if let Some(data) = data {
        set(p, data.into());
    }
}

#[no_mangle]
pub unsafe extern "C" fn object_optional_bytearray_set(ptr: *mut Object, v: QByteArray) {
    (&mut *ptr).set_optional_bytearray(Some(v.convert()));
}
#[no_mangle]
pub unsafe extern "C" fn object_optional_bytearray_set_none(ptr: *mut Object) {
    (&mut *ptr).set_optional_bytearray(None);
}

#[no_mangle]
pub unsafe extern "C" fn object_optional_string_get(
    ptr: *const Object,
    p: *mut c_void,
    set: fn(*mut c_void, QString),
) {
    let data = (&*ptr).optional_string();
    if let Some(data) = data {
        set(p, data.into());
    }
}

#[no_mangle]
pub unsafe extern "C" fn object_optional_string_set(ptr: *mut Object, v: QStringIn) {
    (&mut *ptr).set_optional_string(Some(v.convert()));
}
#[no_mangle]
pub unsafe extern "C" fn object_optional_string_set_none(ptr: *mut Object) {
    (&mut *ptr).set_optional_string(None);
}

#[no_mangle]
pub unsafe extern "C" fn object_string_get(
    ptr: *const Object,
    p: *mut c_void,
    set: fn(*mut c_void, QString),
) {
    let data = (&*ptr).string();
    set(p, data.into());
}

#[no_mangle]
pub unsafe extern "C" fn object_string_set(ptr: *mut Object, v: QStringIn) {
    (&mut *ptr).set_string(v.convert());
}

#[no_mangle]
pub unsafe extern "C" fn object_u64_get(ptr: *const Object) -> u64 {
    (&*ptr).u64()
}

#[no_mangle]
pub unsafe extern "C" fn object_u64_set(ptr: *mut Object, v: u64) {
    (&mut *ptr).set_u64(v);
}

#[no_mangle]
pub unsafe extern "C" fn object_uinteger_get(ptr: *const Object) -> u32 {
    (&*ptr).uinteger()
}

#[no_mangle]
pub unsafe extern "C" fn object_uinteger_set(ptr: *mut Object, v: u32) {
    (&mut *ptr).set_uinteger(v);
}
