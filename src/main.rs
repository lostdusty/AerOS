#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use os::{println, vga_buffer::{Colour, Colours, WRITER}};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    WRITER.lock().set_colour(Colour::new(Colours::BLACK, Colours::LIGHT_CYAN));
    println!("T");
    WRITER.lock().set_colour(Colour::new(Colours::BLACK, Colours::PINK));
    println!("R");
    WRITER.lock().set_colour(Colour::new(Colours::BLACK, Colours::WHITE));
    println!("A");
    WRITER.lock().set_colour(Colour::new(Colours::BLACK, Colours::PINK));
    println!("N");
    WRITER.lock().set_colour(Colour::new(Colours::BLACK, Colours::LIGHT_CYAN));
    println!("S");

    #[cfg(test)]
    test_main();

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os::test_panic_handler(info)
}