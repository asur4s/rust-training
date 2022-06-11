use evdev::Key;
use serde::Deserialize;

#[derive(Clone, Debug)]
// #[serde(untagged)]
pub enum KeyAction{
    Key(Key),
    MultiPurposeKey()
}

