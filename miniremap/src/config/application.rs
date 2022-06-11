use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Application {
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub only: Option<Vec<String>>,
    #[serde(default, deserialize_with = "deserialize_string_or_vec")]
    pub not: Option<Vec<String>>,
}

// TODO: Deserializer 为什么需要生命周期？
fn deserialize_string_or_vec<'de, D>(deserializer: D) -> Result<Option<Vec<String>>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrVec {
        String(String),
        Vec(Vec<String>),
    }

    let vec = match StringOrVec::deserialize(deserializer)? {
        StringOrVec::Vec(vec) => vec,
        StringOrVec::String(string) => vec![string],
    };

    Ok(Some(vec))
}
