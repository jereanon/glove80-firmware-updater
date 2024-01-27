use clap::Parser;
use std::time::Duration;
use std::{fs, thread};
use sysinfo::Disks;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 'f', long)]
    file: String,

    #[arg(short = 'n', long, default_value = "2")]
    times_to_copy: usize,

    #[arg(short = 'd', long, default_value = DEFAULT_FIRMWARE_FILENAME)]
    destination: String,
}

const DEFAULT_FIRMWARE_FILENAME: &str = "GLV80LHBOOT";

pub(crate) fn filename_from_path(path: &str) -> String {
    let path = std::path::Path::new(path);
    let filename = path.file_name().unwrap();
    filename.to_str().unwrap().to_string()
}

fn main() {
    let args = Args::parse();

    let firmware_file = &args.file;
    let times_to_copy = &args.times_to_copy;
    let firmware_filename = filename_from_path(firmware_file);
    let mut times_copied: usize = 0;

    while times_copied < *times_to_copy {
        let disks = Disks::new_with_refreshed_list();
        for disk in &disks {
            if let Some(disk_name) = disk.name().to_str() {
                if disk_name == args.destination {
                    println!(
                        "Copying firmware to {}",
                        disk.mount_point().to_str().unwrap()
                    );
                    fs::copy(
                        firmware_file,
                        format!(
                            "{}/{}",
                            disk.mount_point().to_str().unwrap(),
                            firmware_filename
                        ),
                    )
                    .unwrap();
                    times_copied += 1;
                    println!("Firmware copied to device {:?}!", disk_name);
                }
            }
        }
        thread::sleep(Duration::from_secs(5));
    }

    println!("Firmware update complete!");
}
