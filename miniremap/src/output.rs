use evdev::uinput::{VirtualDevice, VirtualDeviceBuilder};
use evdev::{AttributeSet, Device, Key};
use std::error::Error;

pub fn build_device(base_device: &Device) -> Result<VirtualDevice, Box<dyn Error>> {
    // let mut keys = AttributeSet::<Key>::new();
    // keys.insert(Key::KEY_A);

    let device = VirtualDeviceBuilder::new()?
        .name("minixremap")
        .with_keys(base_device.supported_keys().unwrap())?
        .build()
        .unwrap();

    Ok(device)
}
