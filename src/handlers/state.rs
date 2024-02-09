use std::{
    cmp::{Eq, PartialEq},
    fmt,
    str::FromStr,
};

use color_eyre::eyre::{bail, Error, Result};
use serde::Serialize;
use serde_with::DeserializeFromStr;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, DeserializeFromStr)]
pub enum NormalMode {
    Insert,
    Search,
}

impl fmt::Display for NormalMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Insert => f.write_str("insert"),
            Self::Search => f.write_str("search"),
        }
    }
}

impl FromStr for NormalMode {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "insert" | "input" => Ok(Self::Insert),
            "search" => Ok(Self::Search),
            _ => bail!("Normal mode '{}' cannot be deserialized", s),
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Serialize, DeserializeFromStr)]
pub enum State {
    #[default]
    Dashboard,
    Normal,
    Help,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Dashboard => f.write_str("dashboard"),
            Self::Normal => f.write_str("normal"),
            Self::Help => f.write_str("help"),
        }
    }
}

impl FromStr for State {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "normal" | "default" | "chat" => Ok(Self::Normal),
            "dashboard" | "dash" | "start" => Ok(Self::Dashboard),
            "help" | "commands" => Ok(Self::Help),
            _ => bail!("State '{}' cannot be deserialized", s),
        }
    }
}
