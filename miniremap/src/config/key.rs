use evdev::Key;
use serde::{Deserialize, Deserializer};
use std::{error::Error, str::FromStr};

// 反序列化按键
pub fn deserialize_key<'de, D>(deserializer: D) -> Result<Key, D::Error>
where
    D: Deserializer<'de>,
{
    // 使用反序列化器，获取字段值。
    let key = String::deserialize(deserializer)?;
    // 将字符串字段值转换为 按键对象。
    parse_key(&key).map_err(serde::de::Error::custom)
}

pub fn parse_key(input: &str) -> Result<Key, Box<dyn Error>> {
    // 所有的按钮都转换为大写
    let name = input.to_uppercase();

    // 匹配原始按钮
    if let Ok(key) = Key::from_str(&name) {
        return Ok(key);
    }

    // 简写 KEY_
    if let Ok(key) = Key::from_str(&format!("KEY_{}", name)) {
        return Ok(key);
    }

    let key = match &name[..] {
        "CTRL_R" => Key::KEY_RIGHTCTRL,
        "CTRL_L" => Key::KEY_LEFTCTRL,
        _ => Key::KEY_RESERVED,
    };
    if key != Key::KEY_RESERVED {
        return Ok(key);
    }

    // 没有匹配到按钮
    return Err(format!("unknown key '{}'", input).into());
}

#[test]
fn test_parse_key() {
    assert_eq!(parse_key("KEY_A").unwrap(), Key::KEY_A);
    assert_eq!(parse_key("a").unwrap(), Key::KEY_A);

    assert!(parse_key("foobar").is_err());
}
