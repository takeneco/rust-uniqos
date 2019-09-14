
use core::fmt;

pub struct TextVGA
{
    width:  i32,
    height: i32,
    vram:   *mut u8,

    xpos:   i32,
    ypos:   i32,
}

impl TextVGA
{
    pub fn new(width : i32, height : i32, vram : usize) -> Self {
        TextVGA {
            width,
            height,
            vram : vram as *mut u8,
            xpos : 0,
            ypos : 0,
        }
    }

    pub fn putc(&mut self, c : u8) {
        if c == b'\n' {
            self.xpos = 0;
            self.ypos += 1;
            return;
        }
        let off = (self.width * self.ypos + self.xpos) * 2;
        unsafe {
            let p = self.vram.offset(off as isize);
            *p = c;
            *(p.offset(1)) = 0xf;
        }
        self.xpos += 1;
        if self.xpos >= self.width {
            self.xpos = 0;
            self.ypos += 1;
        }
    }
}

impl fmt::Write for TextVGA {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.bytes() {
            self.putc(c);
        }
        Ok(())
    }
}

