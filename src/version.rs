use clap::{builder::PossibleValue, ValueEnum};
use std::fmt;


#[derive(Debug, Clone)]
pub enum LunarVersion {
    V1_7_10,
    V1_8_9,
    V1_12_2,
    V1_16_5,
    V1_17_1,
    V1_18_1,
    V1_18_2,
    V1_19,
    V1_19_2,
    V1_19_3,
}
impl LunarVersion {
    pub fn get_display_name(&self) -> &str {
        match self {
            LunarVersion::V1_7_10 => "1.7.10",
            LunarVersion::V1_8_9 => "1.8.9",
            LunarVersion::V1_12_2 => "1.12.2",
            LunarVersion::V1_16_5 => "1.16.5",
            LunarVersion::V1_17_1 => "1.17.1",
            LunarVersion::V1_18_1 => "1.18.1",
            LunarVersion::V1_18_2 => "1.18.2",
            LunarVersion::V1_19 => "1.19",
            LunarVersion::V1_19_2 => "1.19.2",
            LunarVersion::V1_19_3 => "1.19.3",
        }
    }
}
impl fmt::Display for LunarVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_display_name())
    }
}
impl ValueEnum for LunarVersion {
    
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Self::V1_7_10,
            Self::V1_8_9,
            Self::V1_12_2,
            Self::V1_16_5,
            Self::V1_17_1,
            Self::V1_18_1,
            Self::V1_18_2,
            Self::V1_19,
            Self::V1_19_2,
            Self::V1_19_3,
        ]
    }
    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            LunarVersion::V1_7_10 => PossibleValue::new("1.7.10"),
            LunarVersion::V1_8_9 => PossibleValue::new("1.8.9"),
            LunarVersion::V1_12_2 => PossibleValue::new("1.12.2"),
            LunarVersion::V1_16_5 => PossibleValue::new("1.16.5"),
            LunarVersion::V1_17_1 => PossibleValue::new("1.17.1"),
            LunarVersion::V1_18_1 => PossibleValue::new("1.18.1"),
            LunarVersion::V1_18_2 => PossibleValue::new("1.18.2"),
            LunarVersion::V1_19 => PossibleValue::new("1.19"),
            LunarVersion::V1_19_2 => PossibleValue::new("1.19.2"),
            LunarVersion::V1_19_3 => PossibleValue::new("1.19.3"),
        })
    }
}
