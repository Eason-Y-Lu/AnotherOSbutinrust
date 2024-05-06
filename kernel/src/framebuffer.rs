use bootloader_api::info::{FrameBuffer, FrameBufferInfo};

pub struct Framebuffer {
    // Dont need the actual FrameBuffer obj cuz this was all that was in it anyway
    fbi: FrameBufferInfo,
    buffer: &'static mut [u8],
    color_locations: ColorLocations,
}

impl Framebuffer {
    pub fn new(fb: &'static mut FrameBuffer) -> Self {
        let fbi = fb.info();
        let buffer = fb.buffer_mut();
        let color_locations = ColorLocations::new(&fbi);

        Self {
            fbi, buffer, color_locations
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
