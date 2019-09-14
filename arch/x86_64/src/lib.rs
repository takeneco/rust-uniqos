
#![no_std]

use core::fmt::Write;
mod textvga;
use self::textvga::TextVGA;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut vga = TextVGA::new(80, 25, 0xb8000);
    vga.putc(b'a');
    //write!(&mut vga, "hello!").unwrap();
    write!(&mut vga, "").unwrap();
    loop {}
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
