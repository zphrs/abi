#![no_std]
#![feature(naked_functions)]

pub mod core;
#[allow(unused_imports)]
pub mod object;

#[cfg(not(feature = "kernel"))]
pub mod alloc;
#[cfg(not(feature = "kernel"))]
pub mod arch;
#[cfg(not(feature = "kernel"))]
pub mod debug;
#[cfg(not(feature = "kernel"))]
pub mod fd;
#[cfg(not(feature = "kernel"))]
pub mod info;
#[cfg(not(feature = "kernel"))]
pub mod io;
#[cfg(not(feature = "kernel"))]
pub mod random;
#[cfg(not(feature = "kernel"))]
pub mod thread;
#[cfg(not(feature = "kernel"))]
pub mod time;

#[allow(
    non_camel_case_types,
    dead_code,
    non_upper_case_globals,
    improper_ctypes
)]
pub mod bindings;
