#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

bootloader_api::entry_point!(kernel_main);

mod framebuffer;
mod interrupts;
mod gdt;

use crate::{framebuffer::{Framebuffer, WRITER}, interrupts::PICS};

#[no_mangle]
#[allow(unused, non_upper_case_globals)]
fn kernel_main(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    let fb = match &mut boot_info.framebuffer {
        bootloader_api::info::Optional::None => panic!(),
        bootloader_api::info::Optional::Some(t) => t
    };

    gdt::init();
    interrupts::init_idt();
    unsafe { PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();

    let mut fb2 = Framebuffer::new(fb);
    x86_64::instructions::interrupts::without_interrupts(|| {
        let mut w = WRITER.lock();

        w.init(fb2);
        w.clear(0xb83c3c);
    });

    println!("TESTING");

    loop {
        x86_64::instructions::hlt();
    }
}

#[allow(unused_imports)]
use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    loop {
        x86_64::instructions::hlt();
    }
}
