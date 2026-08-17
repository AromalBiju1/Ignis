#![no_main]
#![no_std]

use log::info;
use uefi::prelude::*;
use uefi::println;

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();

    info!("Ignis bootloader — starting up");
    info!("Image handle acquired: {:?}", boot::image_handle());

    let _ = system::with_stdout(|stdout| stdout.clear());

    println!("=====================================");
    println!("  Ignis — a minimal UEFI bootloader  ");
    println!("=====================================");
    println!();
    println!("Phase 0 milestone: hello from bare UEFI.");
    println!("Press any key to exit to firmware...");

    let key_event = system::with_stdin(|stdin| stdin.wait_for_key_event().unwrap());
    boot::wait_for_event(&mut [key_event]).unwrap();

    Status::SUCCESS
}