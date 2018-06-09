// Keyboard driver 

use cpuio::inb;

pub fn get_scan_code() -> u8 {
    let mut c: u8 = 0;
    while {
        if inb(0x60) != c {
            c = inb(0x60);
            if c > 0 {
                return c;
            }
        }
    } {} 

}
