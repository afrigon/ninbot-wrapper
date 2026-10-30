use std::str::FromStr;

use anyhow::Error;
use anyhow::bail;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InWorldState {
    Unpaused,
    Paused,
    GameScreenOpen
}

impl FromStr for InWorldState {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "unpaused" => Ok(Self::Unpaused),
            "paused" => Ok(Self::Paused),
            "gamescreenopen" => Ok(Self::GameScreenOpen),
            _ => bail!("unknown in-world state {value:?}")
        }
    }
}
