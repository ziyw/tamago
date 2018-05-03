// struture for entry in page table 

use frame; 
bitflags! {
    pub struct EntryFlags: u64 {
        const PRESENT = 1 << 0;
        const WRITABLE = 1 << 1;
        const USER_ACCESS = 1 << 2;
        const ENABLE_CACHE = 1 << 3;
        const DISABLE_CACHE = 1 << 4;
        const ACCESSED = 1 << 5;
        const DIRTY = 1 << 6;
        const HUGE = 1 << 7;
        const GLOBAL = 1 << 8;
        const NO_EXECUTE = 1 << 63;
    }
}

pub struct Entry(u64);

impl Entry {
    
    pub fn is_unused(&self) -> bool {
        self.0 == 0
    }

    pub fn set_zero(&mut self) {
        self.0 = 0;
    }

    pub fn set(&mut self, frame: frame::Frame, flags: EntryFlags) {
        assert!(frame::frame_to_addr(frame) & !0x000fffff_fffff000 == 0);
        self.0 = (frame::frame_to_addr(frame) as u64 | flags.bits() );
    }


    pub fn get_flags(&self) -> EntryFlags {
        EntryFlags::from_bits_truncate(self.0)
    }

    pub fn get_addr(&self) -> usize {
        self.0 as usize & 0x000fffff_fffff000
    }

    pub fn get_frame(&self) -> Option<frame::Frame> {
        if self.get_flags().contains(EntryFlags::PRESENT) {
            Some(frame::addr_to_frame(self.get_addr())) 
        } else {
            None 
        }
    }

}




