use std::fmt;
use serde::{Deserialize, Deserializer};
use std::str::FromStr;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    pub name: String,
    pub colors: String,
    pub color_identity: String,
    pub defense: Option<String>,
    // 1. Serde now parses the comma-separated string directly into your Vec enum!
    #[serde(deserialize_with = "deserialize_keywords", default)]
    pub keywords: Vec<Keywords>,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, PartialEq, Eq)]
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

impl AsRef<str> for Keywords {
    fn as_ref(&self) -> &str {
        match self {
            Keywords::Flying => "Flying",
            Keywords::Haste => "Haste",
            Keywords::Partner => "Partner",
            Keywords::Reach => "Reach",
            Keywords::Vigilance => "Vigilance",
            Keywords::Transform => "Transform",
            Keywords::Daybound => "Daybound",
            Keywords::Trample => "Trample",
            Keywords::Nightbound => "Nightbound",
        }
    }
}

impl FromStr for Keywords {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "Flying" => Ok(Keywords::Flying),
            "Haste" => Ok(Keywords::Haste),
            "Partner" => Ok(Keywords::Partner),
            "Reach" => Ok(Keywords::Reach),
            "Vigilance" => Ok(Keywords::Vigilance),
            "Transform" => Ok(Keywords::Transform),
            "Daybound" => Ok(Keywords::Daybound),
            "Trample" => Ok(Keywords::Trample),
            "Nightbound" => Ok(Keywords::Nightbound),
            _ => Err(()),
        }
    }
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
