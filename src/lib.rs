#![feature(lang_items)]
#![feature(unique)]
#![feature(const_unique_new)] 
#![feature(ptr_internals)] 
#![feature(abi_x86_interrupt)] 
#![feature(asm)]

#![no_std]
extern crate spin; 
extern crate volatile; 
extern crate multiboot2; 
#[macro_use]
extern crate bitflags; 
extern crate x86_64;
#[macro_use] 
extern crate lazy_static; 
extern crate cpuio;

#[macro_use] 
mod vga; 
mod frame; 
mod page; 
mod pic; 
mod interrupt;
mod clock;
mod proc; 
mod system; 
mod process;
mod keyboard;


#[no_mangle]
pub extern fn kmain(multiboot_info_address: usize) -> ! {

    println!("welcome to tamago");

    unsafe { asm!("cli");}
    pic::init(); 
    interrupt::init();
    clock::init();
    unsafe { asm!("sti");} 
    process::init();
    proc::init();
    //x86_64::instructions::interrupts::int3();
//    x86_64::instructions::interrupts::int3();
//    println!("didn't crash");


    // page::test(); 
/*
    // TODO clean this part or move to frame module 
    let boot_info = unsafe { multiboot2::load(multiboot_info_address) };
    let memory_map_tag = boot_info.memory_map_tag().expect("Memory map tag requried");
    let elf_sections_tag = boot_info.elf_sections_tag().expect("Elf sections tag required");

    let kernel_start: usize = elf_sections_tag.sections().map(|s| s.addr).min().unwrap() as usize;
    let kernel_end: usize = elf_sections_tag.sections().map(|s| s.addr+s.size).max().unwrap() as usize;

    let multiboot_start: usize = multiboot_info_address;
    let multiboot_end: usize  = multiboot_start + (boot_info.total_size as usize);
    
    let mut frameAllocator: frame::FrameAllocator = frame::new(memory_map_tag.memory_areas(), kernel_start, kernel_end,
                                multiboot_start, multiboot_end);


    for _ in 0.. {
        if let Some(frame) = frameAllocator.allocate() {
            println!("New frame num {}", frame);
        }
    }
*/
    loop { } 
}









#[lang = "eh_personality"]
extern fn eh_personality() {
}

#[no_mangle]
#[lang = "panic_fmt"]
extern fn rust_begin_panic(fmt: core::fmt::Arguments, file: &'static str, line: u32) -> ! {
    println!("PANIC in {} at line {}:", file, line);
    println!("{}",fmt);  
    
    loop { } 
}

