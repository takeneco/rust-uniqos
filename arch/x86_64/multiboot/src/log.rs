// Copyright (c) 2019 KATO Takeshi
// Released under the MIT license

/// Logging implements while booting.

use core::fmt;

/// Display logs and messages in VGA.
pub struct TextVGA {
    xpos:   i32,
    ypos:   i32,

    width:  i32,
    height: i32,
    vram:   *mut u8,
}

impl TextVGA
{
    pub fn putc(&mut self, c : u8) {
        if c == b'\n' {
            self.xpos = 0;
            self.ypos += 1;
            return;
        }
        let off = ((self.width * self.ypos + self.xpos) * 2) as isize;
        unsafe {
            let p = self.vram.offset(off);
            *p = c;
            *(p.offset(1)) = 0xf;
        }
        self.xpos += 1;
        if self.xpos >= self.width {
            self.xpos = 0;
            self.ypos += 1;
        }
    }

    pub fn puts(&mut self, s: &str) {
        for c in s.bytes() {
            self.putc(c);
        }
    }
}

/// On memory logs for passing to the kernel.
struct MemLog {
}

pub struct Logger {
    textvga: TextVGA,
}
static mut logger: Logger = Logger {
    textvga: TextVGA {
        xpos: 0,
        ypos: 0,

        width:  80,
        height: 25,
        vram: 0xb8000 as *mut u8,
    },
};

impl fmt::Write for Logger {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.textvga.puts(s);
        Ok(())
    }
}

pub fn log() -> &'static mut Logger {
    unsafe { &mut logger }
}
