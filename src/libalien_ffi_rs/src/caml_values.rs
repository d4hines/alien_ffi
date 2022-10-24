
use caml_ffi_types::{Intnat, Cint, Clong, Value};

extern {
    pub fn long_val(l: Value) -> Clong;
    pub fn int_val(i: Value) -> Cint;
    pub fn nativeint_val(ni: Value) -> Intnat;
    pub fn val_long(l: Clong) -> Value;
    pub fn val_int(i: Cint) -> Value;
    pub fn val_nativeint(ni: Intnat) -> Value;
    pub fn val_true() -> Value;
    pub fn val_false() -> Value;
    pub fn val_unit() -> Value;
}

// Primitives

pub fn caml_unit() -> Value {
    unsafe { val_unit() }
}

pub fn caml_false() -> Value {
    unsafe { val_false() }
}

pub fn caml_true() -> Value {
    unsafe { val_true() }
}

pub fn caml_to_int(i: Value) -> Cint {
    unsafe { int_val(i) }
}

pub fn caml_of_int(i: Cint) -> Value {
    unsafe { val_int(i) }
}

pub fn caml_to_long(l: Value) -> Clong {
    unsafe { long_val(l) }
}

pub fn caml_of_long(l: Clong) -> Value {
    unsafe { val_long(l) }
}

pub fn caml_to_nativeint(nativeint: Value) -> Intnat {
    unsafe { nativeint_val(nativeint) }
}

pub fn caml_of_nativeint(nativeint: Intnat) -> Value {
    unsafe { val_nativeint(nativeint) }
}

// Helpers

fn is_block(x: Value) -> bool {
    (x & 1) == 0
}

fn hd_val(x: Value) -> usize {
    assert!(is_block(x));
    unsafe {
	*(x as *const usize).offset(-1)
    }
}

pub fn tag_of_value(x: Value) -> u8 {
    assert!(is_block(x));
    (hd_val(x) & 0xff) as u8
}

