#![feature(lang_items)]
#![feature(unique)]
#![feature(const_unique_new)] 
#![feature(ptr_internals)] 

#![no_std]
extern crate spin; 

#[lang = "eh_personality"]
extern fn eh_personality() {
}

#[lang = "panic_fmt"]
extern fn rust_begin_panic() -> ! {
    loop { } 
}


mod vga; 


#[no_mangle]
pub extern fn kmain() -> ! {

    vga::test(); 
    loop { } 
}

