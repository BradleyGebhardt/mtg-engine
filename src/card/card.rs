use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{self, Display};
use std::str::FromStr;
use strum_macros::{AsRefStr, EnumString};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    pub name: String,
    #[serde(deserialize_with = "deserialize_colors", serialize_with = "serialize_colors",  default)]
    pub colors: Vec<Colors>,
    pub color_identity: String,
    pub defense: Option<String>,
    // 1. Serde now parses the comma-separated string directly into your Vec enum!
    #[serde(deserialize_with = "deserialize_keywords", default)]
    pub keywords: Vec<Keywords>,
    pub text: String,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} :: {} :: {:?}", self.name, self.text, self.colors)
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, AsRefStr, EnumString)]
pub enum Keywords {
    Flying,
    Reach,
    Vigilance,
    Partner,
    Haste,
    Transform,
    Daybound,
    Trample,
    Nightbound,
}

// 2. Custom deserializer function to split and parse the string
fn deserialize_keywords<'de, D>(deserializer: D) -> Result<Vec<Keywords>, D::Error>
where
    D: Deserializer<'de>,
{
    // Option<String> handles cases where the field is null or a string
    let s: Option<String> = Deserialize::deserialize(deserializer)?;

    let Some(raw_string) = s else {
        return Ok(Vec::new());
    };

    let keywords = raw_string
        .split(',')
        .map(|item| item.trim())
        .filter_map(|item| Keywords::from_str(item).ok())
        .collect();

    Ok(keywords)
}

#[derive(Debug, PartialEq, Eq, Serialize, AsRefStr, EnumString)]
pub enum Colors {
    #[strum(serialize = "W")]
    White,
    #[strum(serialize = "U")]
    Blue,
    #[strum(serialize = "B")]
    Black,
    #[strum(serialize = "R")]
    Red,
    #[strum(serialize = "G")]
    Green,
    #[strum(serialize = "C")]
    Colorless,
}

// 2. Custom deserializer function to split and parse the string
fn deserialize_colors<'de, D>(deserializer: D) -> Result<Vec<Colors>, D::Error>
where
    D: Deserializer<'de>,
{
    // Option<String> handles cases where the field is null or a string
    let s: Option<String> = Deserialize::deserialize(deserializer)?;

    let Some(raw_string) = s else {
        return Ok(Vec::new());
    };

    let colors = raw_string
        .split(',')
        .map(|item| item.trim())
        .filter_map(|item| Colors::from_str(item).ok())
        .collect();

    Ok(colors)
}

fn serialize_colors<S>(colors: &[Colors], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // Map each color variant to its AsRef<str> shorthand ("W", "U", "B", "R", "G", "C")
    let shorthands: Vec<&str> = colors.iter().map(|c| c.as_ref()).collect();
    shorthands.serialize(serializer)
}
