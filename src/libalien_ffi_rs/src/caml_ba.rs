
use caml_ffi_types::{Intnat, Uintnat, Cvoid};

#[repr(C)]
pub struct BigarrayProxy {
    refcount: Intnat,
    data: *mut Cvoid,
    size: Uintnat,
}

#[repr(C)]
pub struct Bigarray {
    pub data: *mut Cvoid,
    pub num_dims: Intnat,
    pub flags: Intnat,
    pub proxy: *const BigarrayProxy,
    pub dim: *const Intnat,
}

// TODO: Deprecate?

// #[allow(non_camel_case_types)]
// pub enum Managed {
//     EXTERNAL = 0,
//     MANAGED = 0x200,
//     MAPPED_FILE = 0x400,
//     MANAGED_MASK = 0x600,
// }

// #[allow(non_camel_case_types)]
// pub enum Kind {
//     FLOAT32 = 0x00,    /* Single-precision floats */
//     FLOAT64 = 0x01,    /* Double-precision floats */
//     SINT8 = 0x02,      /* Signed 8-bit integers */
//     UINT8 = 0x03,      /* Unsigned 8-bit integers */
//     SINT16 = 0x04,     /* Signed 16-bit integers */
//     UINT16 = 0x05,     /* Unsigned 16-bit integers */
//     INT32 = 0x06,      /* Signed 32-bit integers */
//     INT64 = 0x07,      /* Signed 64-bit integers */
//     CAML_INT = 0x08,   /* OCaml-style integers (signed 31 or 63 bits) */
//     NATIVE_INT = 0x09, /* Platform-native long integers (32 or 64 bits) */
//     COMPLEX32 = 0x0a,  /* Single-precision complex */
//     COMPLEX64 = 0x0b,  /* Double-precision complex */
//     CHAR = 0x0c,       /* Characters */
//     KIND_MASK = 0xFF,  /* Mask for kind in flags field */    
// }
