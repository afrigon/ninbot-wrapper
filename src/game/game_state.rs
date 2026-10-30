use std::str::FromStr;

use anyhow::Context;
use anyhow::Error;
use anyhow::bail;

use super::InWorldState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    Title,
    Waiting,
    Wall,
    Generating,
    Previewing,
    InWorld(InWorldState)
}

impl GameState {
    pub fn is_playing(&self) -> bool {
        *self == Self::InWorld(InWorldState::Unpaused)
    }
}

impl FromStr for GameState {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let value = value.trim();
        let (screen, detail) = value.split_once(',').unwrap_or((value, ""));

        match screen {
            "title" => Ok(Self::Title),
            "waiting" => Ok(Self::Waiting),
            "wall" => Ok(Self::Wall),
            "generating" => Ok(Self::Generating),
            "previewing" => Ok(Self::Previewing),
            "inworld" => Ok(Self::InWorld(
                detail
                    .parse()
                    .with_context(|| format!("in game state {value:?}"))?
            )),
            _ => bail!("unknown game state {value:?}")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_every_screen() {
        assert_eq!("title".parse::<GameState>().unwrap(), GameState::Title);
        assert_eq!("wall".parse::<GameState>().unwrap(), GameState::Wall);
        assert_eq!(
            "generating,42".parse::<GameState>().unwrap(),
            GameState::Generating
        );
        assert_eq!(
            "previewing,87".parse::<GameState>().unwrap(),
            GameState::Previewing
        );
        assert_eq!(
            "inworld,gamescreenopen\n".parse::<GameState>().unwrap(),
            GameState::InWorld(InWorldState::GameScreenOpen)
        );
    }

    #[test]
    fn only_unpaused_world_is_playing() {
        assert!(
            "inworld,unpaused"
                .parse::<GameState>()
                .unwrap()
                .is_playing()
        );
        assert!(!"inworld,paused".parse::<GameState>().unwrap().is_playing());
        assert!(!"title".parse::<GameState>().unwrap().is_playing());
    }

    #[test]
    fn rejects_unknown_states() {
        assert!("inworld".parse::<GameState>().is_err());
        assert!("lobby".parse::<GameState>().is_err());
    }
}
