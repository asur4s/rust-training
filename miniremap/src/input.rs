use evdev::Device;

use std::os::unix::io::{AsRawFd, RawFd};
use std::{io, mem, ptr};

pub fn select_device() -> Device {
    // 获取键盘设备，使用 evtest 工具查看
    let device = Device::open("/dev/input/event1");
    return device.unwrap();
}
