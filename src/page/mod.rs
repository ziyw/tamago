mod page_entry; 
mod page_table; 

// TODO keep the const value in mod file 
// page table 
//
pub const TABLE_SIZE: usize = 512; 

pub type PhysicalAddr = usize;
pub type VirtualAddr = usize; 

// will be used for P1, P2, P3, P4 
pub struct Table{
    entries: [page_entry::Entry; TABLE_SIZE],  
    level: u8,
}

// index access 
use core::ops::{Index, IndexMut}; 

impl Index<usize> for Table {
    type Output = page_entry::Entry; 

    fn index(&self, index: usize) -> &page_entry::Entry {
        &self.entries[index] 
    }
}

impl IndexMut<usize> for Table {
    fn index_mut(&mut self, index:usize) -> &mut page_entry::Entry {
        &mut self.entries[index]
    }
}

impl Table {
    pub fn init(&mut self) {
        for entry in self.entries.iter_mut() {
            entry.set_zero(); 
        }
    }

}

impl Table {
    pub fn map(&mut self) {
        if self.level == 4 {
            
        }
    }
}

impl Table {
    pub fn unmap() {} 
}
pub fn test() {
    println!("a new module");
}
