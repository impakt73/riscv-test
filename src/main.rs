#![no_std]
#![no_main]

extern crate panic_halt;

use riscv_rt::entry;

// use `main` as the entry point of this application
// `main` is not allowed to return
#[entry]
fn main() -> ! {
    let base_ptr = 0x1F00 as *mut u8;
    let image_data = unsafe { core::slice::from_raw_parts_mut(base_ptr, 256) };

    loop {
        for y in 0..16 {
            for x in 0..16 {
                let idx = (y << 4) + x;
                //let color = ((x >> 2) & 1) * 255;
                //let color = if (x & y) != 0 { 255 } else { 0 };
                //let lit = ((x >> 2) & 1) ^ ((y >> 2) & 1);

                //image_data[idx] = if lit != 0 { 255 } else { 0 };
                image_data[idx] = (if (((x >> 2) & 1) ^ ((y >> 2) & 1)) != 0 {
                    idx
                } else {
                    0
                }) as u8;
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
