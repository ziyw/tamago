use x86_64::structures::idt::Idt;
use x86_64::structures::idt::ExceptionStackFrame; 
use cpuio::outb;
use pic; 
use clock::KCLOCK;
use process; 

lazy_static! {
    static ref IDT: Idt = {
        let mut idt = Idt::new();
       
        set_up_pit(50);
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.interrupts[0].set_handler_fn(irq0_handler);
        idt.interrupts[1].set_handler_fn(irq1_handler);
        idt 
    };
}

pub fn init() {
    IDT.load();

}

pub fn set_up_pit(frequency: u16) {

    let divisor: u16 = 1193180 / frequency; 
    let l: u8 = (divisor & 0xff) as u8;
    let h: u8 = ((divisor >> 8) & 0xff) as u8;
    
    unsafe {
        outb(0x36,0x43);
        outb(l, 0x40);
        outb(h, 0x40);
    }
}
extern "x86-interrupt" fn irq0_handler(_: &mut ExceptionStackFrame) {
    //println!("timer done");
    KCLOCK.lock().inc();
    if KCLOCK.lock().get_time()%1000 == 0 {
        println!("current time is {}", KCLOCK.lock().get_time());
        process::schedule();
    }
    pic::send_eoi(0);

}

extern "x86-interrupt" fn irq1_handler(_: &mut ExceptionStackFrame) {
    println!("Keyboard");
    pic::send_eoi(1);
}

extern "x86-interrupt" fn breakpoint_handler( stack_frame: &mut ExceptionStackFrame) {
    println!("Exception: BREAKPOINT\n{:#?}", stack_frame);
}



