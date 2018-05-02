
// TODO copy debug trait to copy a strucutre 
struct IDT_entry {
    base_lo: usize,
    sel: usize,
    always_zero: char, 
    flags: char,
    base_hi: usize,
}

struct IDT_ptr {
    limit: usize,
    base: usize, 
}

struct IDT {
    IDT_entry: [IDT_entry; 256],
}

// loading point for asm code 
pub fn load_idt() {

}



pub fn test() {
    load_idt(); 
    /*
    let mut idt: IDT = IDT {
        IDT_entry: [IDT_entry; 256],
    };
    
    let mut idt_ptr: IDT_ptr = IDT_ptr {
        limit: 0,
        base: 0, 
    };

*/ 

}
