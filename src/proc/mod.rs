mod table;
pub mod ipc; 

use core::ptr;

pub struct Proc {
    name: usize,
}

pub struct Table {
    procs: [Proc; 1024],
}
// TODO 
// Table should be global 


pub fn init() {
    
    
    
    // over write memroy location 
    let mut x = 0;
    let y = &mut x as *mut i32;
    let z = 12;
    unsafe {
        ptr::write(y, z);
        assert_eq!(ptr::read(y), 12);
    }

    // clear process table 
    // set up mapping for proc_addr 
    // proc_nr 
    //
}
