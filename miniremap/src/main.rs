use std::error::Error;
use std::path::PathBuf;

mod config;
mod input;
mod select;

fn main() -> Result<(), Box<dyn Error>> {
    // env_logger::init();
    // let config_path: PathBuf =
    //     PathBuf::from("/home/joe/Documents/2022/rust-training/miniremap/config.yml");

    let mut device = input::select_device();
    device.grab()?;

    if select::is_readable(&mut device) {
        println!("event!");
    }

    device.ungrab()?;

    Ok(())
}
