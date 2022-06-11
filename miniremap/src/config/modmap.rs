use crate::config::key_action::KeyAction;
use evdev::Key;
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;

use super::application::Application;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Modmap {
    #[serde(default = "String::new")]
    pub name: String,
    // #[serde(deserialize_with = "deserialize_remap")]
    // pub remap: HashMap<Key, KeyAction>,
    // pub application: Option<Application>,
}

// fn deserialize_remap<'de, D>(deserializer: D) -> Result<HashMap<Key, KeyAction>, D::Error>
// where
//     D: Deserializer<'de>,
// {
//     // #[derive(Deserialize, Eq, PartialEq, Hash)]
//     // struct KeyWrapper(#[serde(deserialize_with = "deserialize_key")] Key);
// }
