#![feature(lang_items)]
#![no_std]

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


