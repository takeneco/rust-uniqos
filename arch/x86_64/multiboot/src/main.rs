
#![feature(lang_items)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[lang = "eh_personality"]
extern fn eh_personality() {}


mod heap;
mod load;
mod log;

//#[no_mangle]
//pub extern "C" fn _start() -> ! {
    //loop {}
//}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
