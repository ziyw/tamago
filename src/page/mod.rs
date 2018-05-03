mod page_entry; 
mod page_table; 

// TODO keep the const value in mod file 
// page table 
//
pub const TABLE_SIZE: usize = 512; 

pub type PhysicalAddr = usize;
pub type VirtualAddr = usize; 

// will be used for P1, P2, P3, P4 
struct Table{
    entry: [page_entry::Entry; TABLE_SIZE],  
}



pub fn test() {
    println!("a new module");
}
