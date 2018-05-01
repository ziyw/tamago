// vga buffer 

pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    LigthMagenta = 13,
    Yellow = 14,
    White = 15, 
}

type char_bits = u8;
type attri_bits = u8;
type vga_entry = u16; 

pub fn get_attribute(back_color: Color, front_color: Color) -> attri_bits {
    ((back_color as u8) << 4) | (front_color as u8) 
}

const BUFFER_WIDTH: usize = 80;
const BUFFER_HEIGHT: usize = 25;

pub struct VGA_Buffer {
    pos: usize, // current cursor position 
    att: attri_bits, // color 
    buffer: *mut vga_entry,  
    // buffer: [[vga_entry; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

impl VGA_Buffer {
    
    pub fn get_vga_entry(&self, b: char_bits) -> vga_entry {
        (b as u16) | ((self.att as u16) << 8) 
    }
    
    pub fn putchar(&mut self, b: char_bits) {
        
        if (b == ('\n' as u8)) {
            self.putline();
        }

        let entry = self.get_vga_entry(b); 
        

        let r = self.pos / BUFFER_WIDTH;
        let c = self.pos % BUFFER_WIDTH; 

        unsafe {
            *self.buffer.add(self.pos) = entry;
        }
        
        self.pos = self.pos + 1;
    }
    
    pub fn putline(&mut self) {
        let empty = self.get_vga_entry(b' ');
        
        let rest = BUFFER_WIDTH - 1 - self.pos % BUFFER_WIDTH;

        for c in 0..rest {
            unsafe {
                *self.buffer.add(self.pos) = empty;
                self.pos += 1;
            }
        }

    }

    pub fn clear(&mut self){
        self.pos = 0;
        let empty = self.get_vga_entry(b' ');

        for r in 0..BUFFER_HEIGHT {
            for c in 0..BUFFER_WIDTH {
                unsafe {
                    *self.buffer.add(r*BUFFER_WIDTH + c) = empty;  
                }
            }
        }
    }


    pub fn print(&mut self, s: &str) {
        for byte in s.bytes() {
            self.putchar(byte);
        }
    }
}


pub fn test() {
    
    let back_color = Color::Black;
    let front_color = Color::LightGreen; 
    let att = get_attribute( back_color, front_color); 

    let mut  vga_buffer: VGA_Buffer = VGA_Buffer {
        pos: 0,
        att: att, 
        buffer: unsafe {(0xb8000 as *mut _) }, 
    }; 
    
    vga_buffer.clear();
    
    vga_buffer.print("Welcome to tamago.");

}
