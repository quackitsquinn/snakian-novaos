

#![no_std]
#![no_main]
#![feature(panic_info_message, custom_test_frameworks)]
#![test_runner(snakian_kernel::testing::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::mem::transmute;
use core::panic::PanicInfo;
use core::fmt::Write;

use pc_keyboard::KeyCode;
use snakian_kernel::interrupts::{init_idt, self};
use snakian_kernel::keyboard_driver::KEYBOARD_DRIVER;
use snakian_kernel::vga_driver::{ColorCode, Color, WRITER};
use snakian_kernel::{serial_println, println, init, eprintln, sleep, print, hardware_interrupts};
use x86_64::instructions;
use x86_64::structures::idt::InterruptDescriptorTable;
use bootloader_api::{BootInfo, entry_point};

#[cfg(not(test))]
#[panic_handler]
pub fn panic_handle(panic: &PanicInfo) -> ! {
    use snakian_kernel::panic_handler;

    panic_handler(panic)
}

#[cfg(test)]
#[panic_handler]
pub fn panic_handle(panic: &PanicInfo) -> ! {
    snakian_kernel::panic_handler(panic);
}

fn entry_point() -> ! {
    init();

    println!("Init complete!");
    println!("Entering loop");
    let mut key: Option<char> = None;
    loop {
        let lock = KEYBOARD_DRIVER.lock();
        if let Some(curchar) = lock.current_char {
            if key != Some(curchar) {
                key = Some(curchar);
                if lock.current_char_as_key == Some(KeyCode::Backspace) {
                    WRITER.lock().backspace();
                } else {
                    print!("{}", key.unwrap());
                }
            }
        } else {
            key = None;
        }
        // This hlt is necessary because the keyboard driver needs to be able to unlock the keyboard
        instructions::hlt();// or asm!("hlt", options(nomem, nostack));
    }
}

entry_point!(kmain);

#[no_mangle]
fn kmain(boot_info: &'static mut BootInfo) -> ! {

    #[cfg(test)]
    test_main();

    entry_point();
}


