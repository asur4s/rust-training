use evdev::Device;

pub fn select_device() -> Device {
    let device = Device::open("/dev/input/event2");
    return device.unwrap();
}
