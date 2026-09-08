#![no_main]
#![no_std]

extern crate alloc;

mod config;
mod fs;

use log::info;
use uefi::prelude::*;
use uefi::println;
use uefi::proto::console::text::{Key, ScanCode};
use uefi::{boot, system, Char16};

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();
    info!("Ignis bootloader — starting up");

    let _ = system::with_stdout(|stdout| stdout.clear());
    println!("=====================================");
    println!("  Ignis — a minimal UEFI bootloader  ");
    println!("=====================================\n");

    let entries = match fs::read_file_as_string("\\ignis.conf") {
        Ok(raw) => config::parse_config(&raw),
        Err(e) => {
            println!("Could not read ignis.conf: {e:?}");
            println!("Falling back to firmware boot menu.");
            boot::stall(3_000_000);
            return Status::SUCCESS;
        }
    };

    if entries.is_empty() {
        println!("ignis.conf found but no valid entries.");
        boot::stall(3_000_000);
        return Status::SUCCESS;
    }

    let enter_key = Char16::try_from('\r').unwrap();
    let mut selected: usize = 0;

    loop {
        let _ = system::with_stdout(|stdout| stdout.clear());
        println!("Select an entry (arrows + Enter):\n");
        for (i, entry) in entries.iter().enumerate() {
            if i == selected {
                println!("  > {}", entry.name);
            } else {
                println!("    {}", entry.name);
            }
        }

        let key_event = system::with_stdin(|stdin| stdin.wait_for_key_event().unwrap());
        boot::wait_for_event(&mut [key_event]).unwrap();
        let key = system::with_stdin(|stdin| stdin.read_key().unwrap());

        match key {
            Some(Key::Special(sc)) => match sc {
                ScanCode::UP if selected > 0 => selected -= 1,
                ScanCode::DOWN if selected + 1 < entries.len() => selected += 1,
                _ => {}
            },
            Some(Key::Printable(c)) if c == enter_key => {
                println!("\nWould boot: {}", entries[selected].name);
                println!("(chainloading not implemented yet — next step)");
                boot::stall(3_000_000);
                return Status::SUCCESS;
            }
            _ => {}
        }
    }
}