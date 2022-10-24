
pub type Cchar = std::os::raw::c_char;
pub type Cint = std::os::raw::c_int;
pub type Clong = std::os::raw::c_long;
pub type Cvoid = std::os::raw::c_void;
pub type Value = i64;

pub type Uintnat = usize;
pub type Intnat = isize;

//const Max_young_wosize : usize = 256;

pub const NO_SCAN_TAG: u8 = 251;
pub const FORWARD_TAG: u8 = 250;
pub const INFIX_TAG: u8 = 249;
pub const OBJECT_TAG: u8 = 248;
pub const CLOSURE_TAG: u8 = 247;
pub const LAZY_TAG: u8 = 246;
pub const ABSTRACT_TAG: u8 = 251;
pub const STRING_TAG: u8 = 252;
pub const DOUBLE_TAG: u8 = 253;
pub const DOUBLE_ARRAY_TAG: u8 = 254;
pub const CUSTOM_TAG: u8 = 255;

