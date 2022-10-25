
use caml_ffi_types::{Intnat, Uintnat, Cint, Cchar, Cvoid, Value};
use caml_values::caml_to_nativeint;
use std::hash::{Hash, Hasher};
use std::convert::TryInto;
use std::collections::hash_map::DefaultHasher;

// DEBUG: use std::io::Write;

extern {
    fn caml_alloc_custom(ops: *const CustomOperations, size: Intnat, used: Intnat, max: Intnat) -> Value;

    fn get_custom_data(v: Value) -> *mut Cvoid;
    fn set_custom_data(v: Value, p: *mut Cvoid) -> Cvoid;

    fn caml_compare_default(v1: Value, v2: Value) -> Cint;
    fn caml_compare_ext_default(v1: Value, v2: Value) -> Cint;
    fn caml_hash_default(v: Value) -> Intnat;
    fn caml_serialize_default(v: Value, bsize_32: *const Uintnat, bsize_64: *const Uintnat) -> Cvoid;
    fn caml_deserialize_default(dst: *mut Cvoid) -> Uintnat;

    // fn caml_serialize_block_1();
    // fn caml_deserialize_block_1();
}

#[repr(C)]
pub struct CustomFixedLength {
    pub bsize_32: Intnat,
    pub bsize_64: Intnat,
}

#[repr(C)]
pub struct CustomOperations {
    pub identifier: *const Cchar,
    pub finalize: extern fn(v: Value),
    pub compare: extern fn (v1: Value, v2: Value) -> Cint,
    pub compare_ext: extern fn (v1: Value, v2: Value) -> Cint,
    pub hash: extern fn (v: Value) -> Intnat,
    pub serialize: extern fn (v: Value, bsize_32: *const Uintnat, *const Uintnat) -> Cvoid,
    pub deserialize: extern fn (*mut Cvoid) -> Uintnat,
    // TODO / FIXME: Version 4.8+ of OCaml ?
    // pub custom_fixed_length: *const CustomFixedLength,
}

unsafe impl Sync for CustomOperations { }

//

pub fn alloc<T>(ops_ptr: *const CustomOperations, v: T) -> Value {
    let data = Box::new(v);
    let size = std::mem::size_of::<*mut Cvoid>() as isize;
    let mem = unsafe { caml_alloc_custom(ops_ptr, size, 0, 1) };
    let data_ptr = Box::into_raw(data);
    unsafe { set_custom_data(mem, data_ptr as *mut Cvoid) };
    mem
}

pub fn acquire<T>(v: Value) -> &'static T {
    let data_ptr = unsafe { get_custom_data(v) as *mut *mut Cvoid };
    let data_ptr = unsafe { (*data_ptr) as *mut T };
    unsafe { data_ptr.as_ref().unwrap() }
}

pub fn acquire_mut<T>(v: Value) -> &'static mut T {
    let data_ptr = unsafe { get_custom_data(v) as *mut *mut Cvoid };
    let data_ptr = unsafe { (*data_ptr) as *mut T };
    unsafe { data_ptr.as_mut().unwrap() }
}

//

pub extern fn finalize<T>(v: Value) {
    // println!("Finalising {} ...", std::any::type_name::<T>());
    // std::io::stdout().flush().unwrap();
    let data_ptr = unsafe { get_custom_data(v) as *mut *mut Cvoid };
    drop(unsafe { Box::from_raw((*data_ptr) as *mut T) })
}

// --

#[no_mangle]
pub extern fn default_compare(v1: Value, v2: Value) -> Cint {
    unsafe { caml_compare_default(v1, v2) }
}

pub extern fn compare<T: Ord>(v1: Value, v2: Value) -> Cint {
    let v1_ptr = unsafe { get_custom_data(v1) as *mut *mut Cvoid };
    let v2_ptr = unsafe { get_custom_data(v2) as *mut *mut Cvoid };
    let v1_ptr = unsafe { (*v1_ptr) as *mut T };
    let v2_ptr = unsafe { (*v2_ptr) as *mut T };
    let v1 = unsafe { v1_ptr.as_ref().unwrap() };
    let v2 = unsafe { v2_ptr.as_ref().unwrap() };
    v1.cmp(&v2) as Cint
}

#[no_mangle]
pub extern fn default_compare_ext(v1: Value, v2: Value) -> Cint {
    unsafe { caml_compare_ext_default(v1, v2) }
}

pub extern fn compare_ext<T: Ord + Hash>(v1: Value, v2: Value) -> Cint {
    let v1: Intnat = hash::<T>(v1).try_into().unwrap();
    let v2 = caml_to_nativeint(v2);
    v1.cmp(&v2) as Cint
}

#[no_mangle]
pub extern fn default_hash(v: Value) -> Intnat {
    unsafe { caml_hash_default(v) }
}

pub extern fn hash<T: Hash>(v: Value) -> Intnat {
    let v_ptr = unsafe { get_custom_data(v) as *mut *mut Cvoid };
    let v_ptr = unsafe { (*v_ptr) as *mut T };
    let v = unsafe { v_ptr.as_ref().unwrap() };
    let mut hasher = DefaultHasher::new();
    v.hash(&mut hasher);
    hasher.finish().try_into().unwrap()
}

#[no_mangle]
pub extern fn default_serialize(v: Value, bsize_32: *const Uintnat, bsize_64: *const Uintnat) -> Cvoid {
    unsafe { caml_serialize_default(v, bsize_32, bsize_64) }
}

#[no_mangle]
pub extern fn default_deserialize(dst: *mut Cvoid) -> Uintnat {
    unsafe { caml_deserialize_default(dst) }
}

