#[allow(dead_code)]
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
pub struct Attribute {
    back_color: Color, 
    front_color: Color, 
}

impl Attribute {
    pub fn get_code(&self) -> u8 {
        ((self.back_color as u8) << 4 | (self.front_color as u8)) 
    }
}

type Entry = u16; 

const BUFFER_WIDTH: usize = 80; 
const BUFFER_HEIGHT: usize = 25; 

pub struct Buffer {
    matrix: [[Entry; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

use core::ptr::Unique; 

struct VGA {
    pos: usize, // current position in thr row 
    color_code: u8, 
    buffer: Unique<Buffer>, 
}


impl VGA {

    
    pub fn put_char(&mut self, b: u8) {
        if b == b'\n' {
            self.put_line();
            self.pos = 0; 
        } else {
            if self.pos == BUFFER_WIDTH {
                self.put_line();
                self.pos = 0;
            }

            let e: Entry = ((b as u16) | (self.color_code as u16) << 8); 
            let col: usize = self.pos;  
            self.buffer().matrix[BUFFER_HEIGHT-1][col] = e; 
            self.pos += 1; 
        }

    }

    pub fn put_line(&mut self) {
        let b: u8 = b' ';
        let e: Entry = ((b as u16) | (self.color_code as u16) << 8); 
        for r in 1..BUFFER_HEIGHT {
            for c in 0..BUFFER_WIDTH {
                let old = self.buffer().matrix[r][c]; 
                self.buffer().matrix[r-1][c] = old;
            }
        }

        for c in 0..BUFFER_WIDTH {
            self.buffer().matrix[BUFFER_HEIGHT-1][c] = e; 
        }
    }

    pub fn buffer(&mut self) -> &mut Buffer {
        unsafe { self.buffer.as_mut() } 
    }


}

pub fn test() {
    let attribute: Attribute = Attribute {
        back_color: Color::Black,
        front_color: Color::LightGreen, 
    }; 

    let color_code = attribute.get_code(); 
    
    let mut vga: VGA = VGA {
        pos: 0,
        color_code: color_code, 
        buffer: unsafe { Unique::new_unchecked(0xb8000 as *mut _) },
    };

    vga.put_char(b'\n');
    vga.put_char(b'A');
    vga.put_char(b'\n');
    vga.put_char(b'B');
    vga.put_char(b'\n');
    vga.put_char(b'C');
}


