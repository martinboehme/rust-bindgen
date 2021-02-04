#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Foo<T>
where
    T: __bindgen_has_inner_type_Associated,
{
    pub member: Foo_SecondAlias,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
}
pub type Foo_FirstAlias<T> =
    <T as __bindgen_has_inner_type_Associated>::Associated;
pub type Foo_SecondAlias = Foo<Foo_FirstAlias>;
impl<T> Default for Foo<T> {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub trait __bindgen_has_inner_type_Associated {
    type Associated: std::fmt::Debug + Default + Copy + Clone;
}
