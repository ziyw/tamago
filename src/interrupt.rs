use x86_64::structures::idt::Idt;
use x86_64::structures::idt::ExceptionStackFrame; 
use cpuio::outb;
use pic; 

lazy_static! {
    static ref IDT: Idt = {
        let mut idt = Idt::new();
        // setting of interruption here 
       
        let divisor: u32 = 1193180 / 50; 
        let l: u16 = (divisor & 0x00ff) as u16;
        let h: u16 = ((divisor >> 8) & 0x00ff) as u16;
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.interrupts[0].set_handler_fn(irq0_handler);
        idt.interrupts[1].set_handler_fn(irq1_handler);
        idt 
    };
}

pub fn init() {
    
    IDT.load();
    // an old interrupt trigger 

}

// set irq handler 
extern "x86-interrupt" fn irq0_handler(_: &mut ExceptionStackFrame) {
    pic::send_EOI(0);
}

extern "x86-interrupt" fn irq1_handler(_: &mut ExceptionStackFrame) {
    println!("keyboard");
    pic::send_EOI(1);
}

extern "x86-interrupt" fn breakpoint_handler( stack_frame: &mut ExceptionStackFrame) {
    println!("Exception: BREAKPOINT\n{:#?}", stack_frame);
}



