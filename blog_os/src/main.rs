#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use blog_os::println;


#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
}

#[test_case]
fn trivial_assert() {
    assert_eq!(1, 1);
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World {}\nWHEN\n", "!");

    blog_os::init();

    #[cfg(test)]
    test_main();

    loop {}
}


