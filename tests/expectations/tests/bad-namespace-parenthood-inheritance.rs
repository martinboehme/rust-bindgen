#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[bindgen_original_name("char_traits")]
pub struct std_char_traits {
    pub _address: u8,
}
impl Default for std_char_traits {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
#[bindgen_original_name("char_traits")]
pub struct __gnu_cxx_char_traits {
    pub _address: u8,
}
