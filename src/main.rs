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
use uefi::boot::LoadImageSource;

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();
    info!("Ignis bootloader — starting up");

    let _ = system::with_stdout(|stdout| stdout.clear());
    println!("=====================================");
    println!("  Ignis — a minimal UEFI bootloader  ");
    println!("=====================================\n");

    let cfg = match fs::read_file_as_string("\\ignis.conf") {
        Ok(raw) => config::parse_config(&raw),
        Err(e) => {
            println!("Could not read ignis.conf: {e:?}");
            println!("Falling back to firmware boot menu.");
            boot::stall(3_000_000);
            return Status::SUCCESS;
        }
    };

    if cfg.entries.is_empty() {
        println!("ignis.conf found but no valid entries.");
        boot::stall(3_000_000);
        return Status::SUCCESS;
    }

    let enter_key = Char16::try_from('\r').unwrap();
    let mut selected: usize = cfg.default;

    // --- Timeout countdown phase (skipped if timeout_secs == 0) ---
    if cfg.timeout_secs > 0 {
        let mut remaining = cfg.timeout_secs;
        'countdown: while remaining > 0 {
            let _ = system::with_stdout(|stdout| stdout.clear());
            println!("Select an entry (arrows + Enter):\n");
            for (i, entry) in cfg.entries.iter().enumerate() {
                if i == selected {
                    println!("  > {}", entry.name);
                } else {
                    println!("    {}", entry.name);
                }
            }
            println!("\nBooting default in {remaining}s... press any key to interrupt.");

            // Poll in ~0.5s ticks so keypress interrupt feels responsive.
            let ticks_per_sec = 2;
            for _ in 0..ticks_per_sec {
                boot::stall(500_000); // 0.5s
                let key = system::with_stdin(|stdin| stdin.read_key().unwrap());
                if key.is_some() {
                    // consume the keypress, then drop into interactive menu below
                    break 'countdown;
                }
            }
            remaining -= 1;
        }

        if remaining == 0 {
           return boot_entry(&cfg.entries[selected]);
}
    }

    // --- Interactive menu loop (reached if timeout=0, or countdown interrupted) ---
    loop {
        let _ = system::with_stdout(|stdout| stdout.clear());
        println!("Select an entry (arrows + Enter):\n");
        for (i, entry) in cfg.entries.iter().enumerate() {
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
                ScanCode::DOWN if selected + 1 < cfg.entries.len() => selected += 1,
                _ => {}
            },
            Some(Key::Printable(c)) if c == enter_key => {
                return boot_entry(&cfg.entries[selected]);
            }
            _ => {}
        }
    }
}

fn boot_entry(entry: &config::BootEntry) -> Status {
    println!("\nBooting: {}", entry.name);

    let image_data = match fs::read_file_bytes(&entry.path) {
        Ok(d) => d,
        Err(e) => {
            println!("Failed to read image: {e:?}");
            boot::stall(3_000_000);
            return Status::LOAD_ERROR;
        }
    };

    let load_source = LoadImageSource::FromBuffer {
        buffer: &image_data,
        file_path: None,
    };

    let image_handle = match boot::load_image(boot::image_handle(), load_source) {
        Ok(h) => h,
        Err(e) => {
            println!("LoadImage failed: {e:?}");
            boot::stall(3_000_000);
            return Status::LOAD_ERROR;
        }
    };

    match boot::start_image(image_handle) {
        Ok(()) => println!("Started image returned normally."),
        Err(e) => println!("StartImage failed: {e:?}"),
    }
    boot::stall(3_000_000);
    Status::SUCCESS
}