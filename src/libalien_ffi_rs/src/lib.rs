
extern crate typed_arena;
extern crate sha2;

pub mod caml_ffi_types;
pub mod caml_opaque;

mod caml_values;
mod caml_ba;

pub use caml_ffi_types::*;
pub use caml_opaque::*;
pub use caml_values::*;

use typed_arena::Arena;
use caml_ba::Bigarray;

//----------------------------------------------------------------------
// C Functions
//----------------------------------------------------------------------

extern {
    pub fn caml_ba_alloc_u8(data: *const Cvoid, ndims: Cint, dims: *const Clong) -> Value;

    // Custom
    pub fn set_finalize(ops: *const CustomOperations, f: extern fn (Value));
}

//----------------------------------------------------------------------
// Retrieve a pointer to CustomOperations (syntactic sugar)
//----------------------------------------------------------------------

pub fn ops(ops: &'static CustomOperations) -> *const CustomOperations {
    ops as *const CustomOperations
}

//----------------------------------------------------------------------

pub fn caml_finalize(ops: &'static CustomOperations, f: extern fn (Value)) {
    unsafe { set_finalize(ops as *const CustomOperations, f) }
}

//----------------------------------------------------------------------
// Rust interface

pub fn caml_ba_data(value: Value) -> *mut Cvoid {
    assert!(tag_of_value(value) == CUSTOM_TAG);
    unsafe {
	let ba_offset = (value as *mut Value).offset(1 as isize);
	let ba = ba_offset as *const Bigarray;
	(*ba).data as *mut Cvoid
    }
}

pub fn caml_to_slice<T>(value: Value) -> &'static [T] {
    assert!(tag_of_value(value) == CUSTOM_TAG);
    unsafe {
	let ba_offset = (value as *mut Value).offset(1 as isize);
	let ba = ba_offset as *const Bigarray;
	let len = (*ba).dim as usize;
	std::slice::from_raw_parts((*ba).data as *const T, len)
    }
}

// Generic external bigarray allocation

pub fn acquire_external<T>(ba: Value) -> &'static T {
    let data_ptr = caml_ba_data(ba) as *mut T;
    unsafe { data_ptr.as_ref().unwrap() }
}

pub fn alloc_external<T>(ext: T) -> Value {
    let ext_box = Box::new(ext);
    let ext_ptr = Box::into_raw(ext_box);
    let size = std::mem::size_of::<T>();
    let dims = &[size as Clong];
    unsafe { caml_ba_alloc_u8(ext_ptr as *const Cvoid, 1, dims.as_ptr()) }
}

pub fn free_external<T>(ba: Value) -> () {
    let data_ptr = caml_ba_data(ba) as *mut T;
    drop(unsafe { Box::from_raw(data_ptr) })
}

// Arena allocation

pub fn acquire_ba_arena(arena: Value) -> &'static Arena<Vec<u8>> {
    acquire_external::<Arena<Vec<u8>>>(arena)
}

pub fn arena_alloc_ba(arena: &'static Arena<Vec<u8>>, msg: Vec<u8>) -> Value {
    let size = std::mem::size_of::<u8>() * msg.len();
    let dims = &[size as Clong];
    let mem = arena.alloc(msg);
    unsafe { caml_ba_alloc_u8(mem.as_ptr() as *const Cvoid, 1, dims.as_ptr()) }
}

//----------------------------------------------------------------------
// OCaml interface

#[no_mangle]
pub extern fn arena_alloc(_unit: Value) -> Value {
    alloc_external::<Arena<Vec<u8>>>(Arena::new())
}

#[no_mangle]
pub extern fn arena_free(arena: Value) -> Value {
    free_external::<Arena<Vec<u8>>>(arena);
    caml_unit()
}

