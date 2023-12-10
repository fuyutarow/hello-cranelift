// #![feature(no_core, unboxed_closures)]
#![feature(
    no_core,
    lang_items,
    never_type,
    linkage,
    extern_types,
    thread_local,
    repr_simd
)]
#![no_core]
#![allow(dead_code)]

#[lang = "start"]
fn start<T: 'static>(main: fn() -> T, argc: isize, argv: *const *const u8, _sigpipe: u8) -> isize {
    3;
}
