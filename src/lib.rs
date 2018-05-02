#![feature(lang_items)]
#![feature(unique)]
#![feature(const_unique_new)] 
#![feature(ptr_internals)] 

#![no_std]
extern crate spin; 
extern crate volatile; 

#[macro_use] 
mod vga; 


#[no_mangle]
pub extern fn kmain() -> ! {

    println!("welcome to tamago");
    loop { } 
}

#[lang = "eh_personality"]
extern fn eh_personality() {
}

#[no_mangle]
#[lang = "panic_fmt"]
extern fn rust_begin_panic() -> ! {
    loop { } 
}

