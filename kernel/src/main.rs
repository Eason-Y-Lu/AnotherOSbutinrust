#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

bootloader_api::entry_point!(kernel_main);

mod framebuffer;
mod interrupts;

use crate::framebuffer::Framebuffer;

#[no_mangle]
fn kernel_main(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    let fb: &mut bootloader_api::info::FrameBuffer;

    match &mut boot_info.framebuffer {
        bootloader_api::info::Optional::None => loop {},
        bootloader_api::info::Optional::Some(t) => fb = t
    };

    interrupts::init_idt();

    let mut fb2 = Framebuffer::new(fb);
    fb2.clear(0xb83c3c);

    const size: usize = 50;
    const buf_size: usize = size * size * 3;
    let buf: &[u8; buf_size] = &[255; buf_size];
    fb2.cpy_buff(buf, (size, size), (100, 100));

    unsafe {
        // *(0xdeadbeef as *mut u8) = 42;
    }

    loop {
        x86_64::instructions::hlt();
    }
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
