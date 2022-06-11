use evdev::Device;

use std::os::unix::io::{AsRawFd, RawFd};
use std::{io, mem, ptr};

pub fn select_device() -> Device {
    let device = Device::open("/dev/input/event1");
    return device.unwrap();
}
