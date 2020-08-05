#![no_std]
#![no_main]

extern crate panic_halt;

use riscv_rt::entry;

// use `main` as the entry point of this application
// `main` is not allowed to return
#[entry]
fn main() -> ! {
    let reg_base_ptr = 0x100000 as *const u32;
    let regs = unsafe { core::slice::from_raw_parts(reg_base_ptr, 1024) };

    let fb_base_addr = regs[1] as *mut u32;

    let fb_width_log2 = (regs[2] & 0x7) + 1;
    let fb_height_log2 = ((regs[2] >> 3) & 0x7) + 1;

    let fb_width = 1 << fb_width_log2;
    let fb_height = 1 << fb_height_log2;

    let fb_data = unsafe { core::slice::from_raw_parts_mut(fb_base_addr, (fb_width * fb_height) as usize) };

    let mut time = 0;

    loop {
        for y in 0..fb_height {
            for x in 0..fb_width {
                let idx = y * fb_width + x;

                let tile_x = (x + time) >> 3;
                let tile_y = y >> 3;
                let tile_idx = tile_y * (fb_width >> 3) + tile_x;

                //let color = ((x >> 2) & 1) * 255;
                //let color = if (x & y) != 0 { 255 } else { 0 };
                //let lit = ((x >> 2) & 1) ^ ((y >> 2) & 1);

                //image_data[idx] = if lit != 0 { 255 } else { 0 };
                fb_data[idx as usize] = if ((((x + (time >> 1)) >> 4) & 1) ^ ((y >> 4) & 1)) != 0 || true {
                    let color_val: u32 = 0xff << ((tile_idx & 0x3) << 3);
                    let r = (color_val & 0xff) as u8;
                    let g = ((color_val >> 8) & 0xff) as u8;
                    let b = ((color_val >> 16) & 0xff) as u8;
                    u32::from_le_bytes([r, g, b, 0xff])
                    //0xffffffff
                } else {
                    0xff000000
                };
                //image_data[idx] = (idx & 255) as u8;
            }
        }

        /*
        let num_items = 16;
        for idx in 0..num_items {
            image_data[idx] = (((idx as f32) / ((num_items - 1) as f32)) * 255.0) as u8;
        }
        */

        // End the program by executing the wfi instruction
        unsafe {
            riscv::asm::wfi();
        }

        time += 1;
    }
}
