//Memory allocation 

pub const FRAME_SIZE: usize = 4096;
type Frame = usize; // Frame number  

pub fn addr_to_frame(address: usize) -> Frame {
    address / FRAME_SIZE 
}


use multiboot2::{MemoryAreaIter, MemoryArea};

pub struct FrameAllocator{
    memory_areas: MemoryAreaIter, 
    current_area: Option<&'static MemoryArea>,
    current_frame: Frame, 
    
    kernel_start: Frame, 
    kernel_end: Frame, 
    
    multiboot_start: Frame, 
    multiboot_end: Frame, 
}

impl FrameAllocator {
    pub fn init(&mut self) {
        loop {
            if let Some(area) = self.memory_areas.next() {
                let mut frame = addr_to_frame(area.base_addr as usize);
                let area_end = addr_to_frame((area.base_addr + area.length) as usize);
                while frame <= area_end && !self.valid(frame) {
                    frame += 1;
                }

                if self.valid(frame) {
                    self.current_area = Some(area); 
                    self.current_frame = frame; 
                    break;
                }
            
            }
        }

    }
    
    pub fn valid(&self, frame: Frame) -> bool {
        if frame >= self.kernel_start && frame <= self.kernel_end {
            return false;
        }

        if frame >= self.multiboot_start && frame <= self.multiboot_end {
            return false; 
        }

        if let Some(area) = self.current_area {
            let start = addr_to_frame(area.base_addr as usize);
            let end = addr_to_frame((area.base_addr + area.length) as usize);
            
            if frame > end {
                return false;
            }
        }

        true 
    }

    pub fn allocate(&mut self) -> Option<Frame> {
        if self.valid(self.current_frame + 1) {
            self.current_frame += 1;
            return Some(self.current_frame);
        }   

        let mut next_frame = self.current_frame; 
        while !self.valid(next_frame) {
            if self.in_valid_area(next_frame) {
               next_frame += 1;
            } else {
            
                if let Some(area) = self.memory_areas.next() {
                    self.current_area = Some(area); 
                
                } else {
                    self.current_area = None;
                    return None; 
                }
            }
        }
        None 
    }

    pub fn in_valid_area(&self, frame: Frame) -> bool {
        if let Some(area) = self.current_area {
            let start = addr_to_frame(area.base_addr as usize);
            let end = addr_to_frame((area.base_addr + area.length) as usize);
            if frame >= start && frame <= end {
                return true
            }
        }
        false 
    }




    pub fn deallocate(&mut self) {
    
    }

}

pub fn new(memory_areas: MemoryAreaIter, kernel_start: usize, kernel_end: usize,
            multiboot_start: usize, multiboot_end: usize) -> FrameAllocator {
    
    let mut frameAllocator: FrameAllocator = FrameAllocator {
        memory_areas: memory_areas,
        current_area: None, 
        current_frame: addr_to_frame(0),
        kernel_start: addr_to_frame(kernel_start),
        kernel_end: addr_to_frame(kernel_end),
        multiboot_start: addr_to_frame(multiboot_start),
        multiboot_end: addr_to_frame(multiboot_end),
    };

    frameAllocator.init(); 
    frameAllocator 
}

