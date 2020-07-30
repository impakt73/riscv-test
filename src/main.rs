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

    let fb_base_addr = regs[1] as *mut u8;

    let fb_width_log2 = (regs[2] & 0x7) + 1;
    let fb_height_log2 = ((regs[2] >> 3) & 0x7) + 1;

    let fb_width = 1 << fb_width_log2;
    let fb_height = 1 << fb_height_log2;

    let fb_data = unsafe { core::slice::from_raw_parts_mut(fb_base_addr, (fb_width * fb_height) as usize) };

    loop {
        for y in 0..fb_height {
            for x in 0..fb_width {
                let idx = y * fb_width + x;
                //let color = ((x >> 2) & 1) * 255;
                //let color = if (x & y) != 0 { 255 } else { 0 };
                //let lit = ((x >> 2) & 1) ^ ((y >> 2) & 1);

                //image_data[idx] = if lit != 0 { 255 } else { 0 };
                fb_data[idx as usize] = (if (((x >> 2) & 1) ^ ((y >> 2) & 1)) != 0 {
                    idx & 255
                } else {
                    0
                }) as u8;
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
    }
}
