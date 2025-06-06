#![feature(prelude_import)]
#![no_std]
#![no_main]
#[prelude_import]
use core::prelude::rust_2024::*;
#[macro_use]
extern crate core;
extern crate compiler_builtins as _;
const _: fn() = main;
#[no_mangle]
#[allow(unused)]
fn main() {
    let out = (|| {
        {
            let (y): (u32) = nexus_rt::read_private_input::<(u32)>()
                .expect("Failed to read public input");
            {
                {
                    let (x): (u32) = nexus_rt::read_private_input::<(u32)>()
                        .expect("Failed to read public input");
                    { { x * y } }
                }
            }
        }
    })();
    nexus_rt::write_public_output::<u32>(&out).expect("Failed to write output");
}
