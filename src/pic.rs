
use cpuio::{inb, outb}; 

pub fn init() {
    let master_offset: u8 = 0x20;
    let slave_offset: u8 = 0x28;
    remap(master_offset, slave_offset);
}

const PIC1: u16 = 0x20;
const PIC2: u16 = 0xA0;
const PIC1_COMMAND: u16 = PIC1;
const PIC1_DATA: u16 = PIC1 + 1;
const PIC2_COMMAND: u16 = PIC2;
const PIC2_DATA: u16 = PIC2 + 1;

const PIC_EOI: u16 = 0x20; 

const ICW1_ICW4: u16 = 0x01; 
const ICW1_SINGLE: u16 = 0x02; 
const ICW1_INTERVAL4: u16 = 0x04;
const ICW1_LEVEL: u16 = 0x08;
const ICW1_INIT: u16 = 0x10;

const ICW4_8086: u8 = 0x01;
const ICW4_AUTO: u8 = 0x02;
const ICW4_BUF_SLAVE: u8 = 0x08;
const ICW4_BUF_MASTER: u8 = 0x0c;
const ICW4_SFNM: u8 = 0x10; 

pub fn remap(offset1: u8, offset2: u8) {
    unsafe {
        let a1: u8 = inb(PIC1_DATA);
        let a2: u8 = inb(PIC2_DATA);
        
        outb((ICW1_INIT+ICW1_ICW4) as u8, PIC1_COMMAND);
        outb((ICW1_INIT+ICW1_ICW4) as u8, PIC2_COMMAND);

        outb(offset1, PIC1_DATA);
        outb(offset2, PIC2_DATA);

        outb(4, PIC1_DATA);
        outb(2, PIC2_DATA);

        outb(ICW4_8086, PIC1_DATA);
        outb(ICW4_8086, PIC2_DATA);

        outb(a1, PIC1_DATA);
        outb(a2, PIC2_DATA);
    }
}
pub fn send_EOI(irq: u8) {
    if irq >= 8 {
        unsafe {
            outb(PIC_EOI as u8, PIC2_COMMAND);
        }
    }

    unsafe {
        outb( PIC_EOI as u8, PIC1_COMMAND);
    }
    
}
