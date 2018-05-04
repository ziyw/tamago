use x86_64::structures::idt::Idt;
use x86_64::structures::idt::ExceptionStackFrame; 
use cpuio::outb;
use pic; 
use clock::KCLOCK;

lazy_static! {
    static ref IDT: Idt = {
        let mut idt = Idt::new();
       
        let divisor: u32 = 1193180 / 50; 
        let l: u16 = (divisor & 0x00ff) as u16;
        let h: u16 = ((divisor >> 8) & 0x00ff) as u16;
        
       /* unsafe {
            asm!("sti");
            //outb(0x43, 0x36);
            //outb(0x40, l);
            //outb(0x40, h);
        }
        */
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.interrupts[0].set_handler_fn(irq0_handler);
        idt.interrupts[1].set_handler_fn(irq1_handler);
        idt 
    };
}

pub fn init() {
    IDT.load();

}


extern "x86-interrupt" fn irq0_handler(_: &mut ExceptionStackFrame) {
    //println!("timer done");
    KCLOCK.lock().inc();
    // println!("current time is {}", KCLOCK.lock().get_time());
    pic::send_eoi(0);

}

extern "x86-interrupt" fn irq1_handler(_: &mut ExceptionStackFrame) {
    println!("Keyboard");
    pic::send_eoi(1);
}

extern "x86-interrupt" fn breakpoint_handler( stack_frame: &mut ExceptionStackFrame) {
    println!("Exception: BREAKPOINT\n{:#?}", stack_frame);
}



