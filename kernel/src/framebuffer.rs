use core::{fmt, slice};

use spin::Mutex;
use bootloader_api::info::{FrameBuffer, FrameBufferInfo};

lazy_static! {
    pub static ref WRITER: Mutex<FramebufferWriter> = Mutex::new(FramebufferWriter::new());
}

pub struct Framebuffer {
    // Dont need the actual FrameBuffer obj cuz this was all that was in it anyway
    fbi: FrameBufferInfo,
    buffer: &'static mut [u8],
    color_locations: ColorLocations,
}

#[allow(dead_code)]
impl Framebuffer {
    pub fn new(fb: &'static mut FrameBuffer) -> Self {
        let fbi = fb.info();
        let buffer = fb.buffer_mut();
        let color_locations = ColorLocations::new(&fbi);

        Self {
            fbi, buffer, color_locations
        }
    }

    // Just the settings for my pc, but they'll be overwritten so it doesn't really matter
    fn default() -> Self {
        let fbi = FrameBufferInfo {
            byte_len: 3145728,
            width: 1024,
            height: 780,
            pixel_format: bootloader_api::info::PixelFormat::Bgr,
            bytes_per_pixel: 4,
            stride: 1024,
        };

        Self {
            fbi,
            buffer: unsafe { slice::from_raw_parts_mut(0x80000000 as *mut u8, fbi.byte_len) },
            color_locations: ColorLocations::new(&fbi),
        }
    }

    pub fn clear(&mut self, clear_color: u64) {
        // Need to shift it by 4 * # of bytes, cuz its BIT shift not BYTE shift >:c
        // now that it works properly, there is no way that it will fail, so we can risk the hang
        let red: u8 = ((clear_color & 0xFF_00_00) >> (4 * 4)).try_into().unwrap();
        let green: u8 = ((clear_color & 0x00_FF_00) >> (2 * 4)).try_into().unwrap();
        let blue: u8 = (clear_color & 0x00_00_FF).try_into().unwrap();

        for i in 0..self.fbi.height {
            for j in 0..self.fbi.width {
                // Stride is in pixels, so we need to convert it to bytes
                let index = i * self.fbi.stride * self.fbi.bytes_per_pixel + j * self.fbi.bytes_per_pixel;

                self.buffer[index + self.color_locations.r] = red;
                self.buffer[index + self.color_locations.g] = green;
                self.buffer[index + self.color_locations.b] = blue;
            }
        }
    }

    // Copies a pixel buffer into the main framebuffer, so that windows can have
    // seperate framebuffers that only they write to
    pub fn cpy_buff(&mut self, buffer: &'static [u8], buffer_size: (usize, usize), start: (usize, usize)) {
        if start.0 > self.fbi.width || start.1 > self.fbi.height { return }

        // calculate the buffer-relative stop indexes so that the window can go offscreen
        let x_stop_index = if start.0 + buffer_size.0 <= self.fbi.width { buffer_size.0 } else { start.0 + buffer_size.0 - self.fbi.width };
        let y_stop_index = if start.1 + buffer_size.1 <= self.fbi.width { buffer_size.1 } else { start.1 + buffer_size.1 - self.fbi.width };

        for i in 0..y_stop_index {
            for j in 0..x_stop_index {
                let index = (i + start.1) * self.fbi.stride * self.fbi.bytes_per_pixel +
                    (j + start.0) * self.fbi.bytes_per_pixel;
                let buffer_index = i * 3 * buffer_size.0 + j * 3;

                self.buffer[index + self.color_locations.r] = buffer[buffer_index];
                self.buffer[index + self.color_locations.g] = buffer[buffer_index + 1];
                self.buffer[index + self.color_locations.b] = buffer[buffer_index + 2];
            }
        }
    }

    pub fn write_pixel_byte_opacive(&mut self, x: usize, y: usize, byte: u16, opacity: u16) {
        if opacity == 0 { return }

        let i = y * self.fbi.stride * self.fbi.bytes_per_pixel + x * self.fbi.bytes_per_pixel;

        let r = self.buffer[i + self.color_locations.r] as u16;
        let g = self.buffer[i + self.color_locations.g] as u16;
        let b = self.buffer[i + self.color_locations.b] as u16;

        self.buffer[i + self.color_locations.r] = ((byte * opacity + (255 - opacity) * r) / 255) as u8;
        self.buffer[i + self.color_locations.g] = ((byte * opacity + (255 - opacity) * g) / 255) as u8;
        self.buffer[i + self.color_locations.b] = ((byte / 2 * opacity + (255 - opacity) * b) / 255) as u8;
    }

    pub fn clone(&mut self) -> Self {
        let c = &self.color_locations;
        let r = c.r;
        let g = c.g;
        let b = c.b;

        let buffer = unsafe { core::slice::from_raw_parts_mut(self.buffer.as_ptr() as *mut u8, self.fbi.byte_len) };

        Self {
            fbi: FrameBufferInfo { ..self.fbi },
            buffer,
            color_locations: ColorLocations { r, g, b }
        }
    }
}

struct ColorLocations {
    pub r: usize,
    pub g: usize,
    pub b: usize,
}

impl ColorLocations {
    pub fn new(fbi: &FrameBufferInfo) -> Self {
        match fbi.pixel_format {
            bootloader_api::info::PixelFormat::Rgb => Self { r: 0, g: 1, b: 2 },
            bootloader_api::info::PixelFormat::Bgr => Self { r: 2, g: 1, b: 0 },
            bootloader_api::info::PixelFormat::U8 => Self { r: 0, g: 0, b: 0 },
            bootloader_api::info::PixelFormat::Unknown { red_position, green_position, blue_position } =>
                Self { r: red_position as usize, g: green_position as usize, b: blue_position as usize },
            // Assume its BGR if it isnt anything else, just because its BGR on my computer
            _ => Self { r: 2, g: 1, b: 0 }
        }
    }
}

use lazy_static::lazy_static;
use noto_sans_mono_bitmap::{get_raster, get_raster_width, FontWeight, RasterHeight, RasterizedChar};
use x86_64::instructions::interrupts;

const LINE_SPACING: usize = 2;
const LETTER_SPACING: usize = 0;

const BORDER_PADDING: usize = 1;

mod font_constants {
    use super::*;

    pub const FONT_WEIGHT: FontWeight = FontWeight::Regular;
    pub const CHAR_RASTER_HEIGHT: RasterHeight = RasterHeight::Size16;
    pub const CHAR_RASTER_WIDTH: usize = get_raster_width(FONT_WEIGHT, CHAR_RASTER_HEIGHT);
    pub const BACKUP_CHAR: char = '�';
}

fn get_char_raster(c: char) -> RasterizedChar {
    fn get(c: char) -> Option<RasterizedChar> {
        get_raster(c, font_constants::FONT_WEIGHT, font_constants::CHAR_RASTER_HEIGHT)
    }
    get(c).unwrap_or_else(|| get(font_constants::BACKUP_CHAR).expect("BROO EXPLODED. get the raster of the backup char"))
}

pub struct FramebufferWriter {
    x: usize,
    y: usize,
    fb: Framebuffer,
}

#[allow(dead_code)]
impl FramebufferWriter {
    pub fn new() -> Self {
        Self {
            x: BORDER_PADDING,
            y: BORDER_PADDING,
            fb: Framebuffer::default(),
        }
    }

    pub fn init(&mut self, fb: Framebuffer) {
        self.fb = fb;
    }

    pub fn clear(&mut self, clear_color: u64) {
        self.fb.clear(clear_color);
    }

    pub fn cpy_buff(&mut self, buffer: &'static [u8], buffer_size: (usize, usize), start: (usize, usize)) {
        self.fb.cpy_buff(buffer, buffer_size, start);
    }

    pub fn write_pixel_byte_opacive(&mut self, x: usize, y: usize, byte: u8, opacity: u8) {
        self.fb.write_pixel_byte_opacive(x, y, byte as u16, opacity as u16);
    }

    fn newline(&mut self) {
        self.y += font_constants::CHAR_RASTER_HEIGHT.val() + LINE_SPACING;
        self.carriage_return();
    }

    fn carriage_return(&mut self) {
        self.x = BORDER_PADDING;
    }

    fn write_char(&mut self, c: char) {
        match c {
            '\n' => self.newline(),
            '\r' => self.carriage_return(),
            c => {
                let x = self.x + font_constants::CHAR_RASTER_WIDTH;

                if x > self.fb.fbi.width {
                    self.newline();
                }

                let y = self.y + font_constants::CHAR_RASTER_HEIGHT.val() + BORDER_PADDING;

                if y > self.fb.fbi.height {
                    self.fb.clear(0);
                }

                self.write_rendered_char(get_char_raster(c));
            }
        }
    }

    fn write_rendered_char(&mut self, c: RasterizedChar) {
        for (y, row) in c.raster().iter().enumerate() {
            for (x, byte) in row.iter().enumerate() {
                self.fb.write_pixel_byte_opacive(self.x + x, self.y + y, *byte as u16, *byte as u16);
            }
        }

        self.x += c.width() + LETTER_SPACING;
    }
}

impl fmt::Write for FramebufferWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.write_char(c);
        }
        Ok(())
    }

    // fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> fmt::Result {
    //     let _ = self.write_str(args.as_str().unwrap());
    //     Ok(())
    // }
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg: tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! print {
    ($($arg: tt)*) => ($crate::framebuffer::_print(format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use fmt::Write;

    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}
