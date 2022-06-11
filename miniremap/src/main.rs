use std::error::Error;
use std::path::PathBuf;

use evdev::{Device, EventType};

mod config;
mod input;
mod output;
mod select;

fn event_loop(input_device: &mut Device) -> Result<(), Box<dyn Error>> {
    let mut output_device = output::build_device(input_device).unwrap();

    // 捕获事件
    for _ in 0..5 {
        if !select::is_readable(input_device) {
            continue;
        }
        for event in input_device.fetch_events().unwrap() {
            if event.event_type() == EventType::KEY {
                println!("event: {:?}", event);
            }
            // 虚拟设备 emit 有什么作用？
            // 将事件提交给操作系统。
            output_device.emit(&[event]).unwrap();
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // env_logger::init();
    // let config_path: PathBuf =
    //     PathBuf::from("/home/joe/Documents/2022/rust-training/miniremap/config.yml");

    let mut device = input::select_device();
    device.grab()?;
    event_loop(&mut device)?;
    device.ungrab()?;

    Ok(())
}
