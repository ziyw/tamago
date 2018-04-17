
#![feature(lang_items)] // required for defining the panic handler
#![feature(const_fn)]

#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

extern crate volatile; 
#[macro_use]
extern crate lazy_static;
extern crate spin; 

#[macro_use]
mod vga_buffer; 

#[lang = "panic_fmt"] // define a function that should be called on panic
#[no_mangle]
pub extern "C" fn rust_begin_panic(
    _msg: core::fmt::Arguments,
    _file: &'static str,
    _line: u32,
    _column: u32,
) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello sailor{}","!");
    loop {} 
}


