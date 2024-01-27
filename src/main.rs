//!
//! A simple command line utility to update the firmware on a [Glove80]((https://www.moergo.com/) device.
//!
//! # Examples
//!
//! Run the firmware updater with default values:
//! ```bash
//! glove80-firmware-updater -f firmware.uf2
//! ```
//! Run the firmware updater with a full path to the firmware file:
//! ```bash
//! glove80-firmware-updater -f /home/user/firmware.uf2
//! ```
//!
//! Run the firmware updater with non-default values:
//! ```bash
//! glove80-firmware-updater -f firmware.uf2 -l GLV80LHBOOT -r GLV80RHBOOT
//! ```
//!

use clap::Parser;
use std::path::PathBuf;
use std::process::exit;
use std::time::Duration;
use std::{fs, thread};
use sysinfo::Disks;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 'f', long)]
    file: String,

    #[arg(short = 'l', long, default_value = "GLV80LHBOOT")]
    left_hand_destination: String,

    #[arg(short = 'r', long, default_value = "GLV80RHBOOT")]
    right_hand_destination: String,
}

const DEFAULT_SLEEP_DURATION: Duration = Duration::from_secs(5);

fn main() {
    let args = Args::parse();

    // check that the firmware file exists both relative to the executable and as a full path
    let firmware_file = PathBuf::from(&args.file);
    if !firmware_file.exists() {
        println!("Error: {} does not exist!", firmware_file.to_str().unwrap());
        exit(1);
    }

    // get the filename from the path
    let firmware_filename = firmware_file
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    let mut remaining_destinations =
        vec![&args.left_hand_destination, &args.right_hand_destination];

    let mut disks = Disks::new_with_refreshed_list();

    while !remaining_destinations.is_empty() {
        disks.refresh_list();
        for disk in &disks {
            if let Some(disk_name) = disk.name().to_str() {
                if remaining_destinations.contains(&&disk_name.to_string()) {
                    println!(
                        "Copying firmware to {}",
                        disk.mount_point().to_str().unwrap()
                    );
                    fs::copy(&firmware_file, disk.mount_point().join(&firmware_filename)).unwrap();

                    println!("Firmware copied to device {:?}!", disk_name);
                    remaining_destinations.retain(|&d| d != disk_name);

                    if remaining_destinations.is_empty() {
                        println!("Firmware update complete!");
                        exit(0);
                    }
                }
            }
        }
        thread::sleep(DEFAULT_SLEEP_DURATION);
    }
}
