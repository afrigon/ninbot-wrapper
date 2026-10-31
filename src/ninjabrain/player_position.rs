use serde::Deserialize;
use serde::Deserializer;
use serde::de::Error;

use super::Dimension;

#[derive(Debug, Clone)]
pub struct PlayerPosition {
    pub x_in_overworld: f64,
    pub z_in_overworld: f64,
    pub horizontal_angle: f64,
    pub is_in_overworld: bool,
    pub is_in_nether: bool
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct PartialPlayerPosition {
    x_in_overworld: Option<f64>,
    z_in_overworld: Option<f64>,
    horizontal_angle: Option<f64>,
    is_in_overworld: Option<bool>,
    is_in_nether: Option<bool>
}

impl PlayerPosition {
    pub fn dimension(&self) -> Dimension {
        if self.is_in_nether {
            Dimension::Nether
        } else if self.is_in_overworld {
            Dimension::Overworld
        } else {
            Dimension::End
        }
    }

    // Ninjabrain Bot sends an empty object, not null, when it has no position.
    pub fn deserialize_optional<'de, D: Deserializer<'de>>(
        deserializer: D
    ) -> Result<Option<Self>, D::Error> {
        let Some(partial) = Option::<PartialPlayerPosition>::deserialize(deserializer)? else {
            return Ok(None);
        };

        match partial {
            PartialPlayerPosition {
                x_in_overworld: Some(x_in_overworld),
                z_in_overworld: Some(z_in_overworld),
                horizontal_angle: Some(horizontal_angle),
                is_in_overworld: Some(is_in_overworld),
                is_in_nether: Some(is_in_nether)
            } => Ok(Some(Self {
                x_in_overworld,
                z_in_overworld,
                horizontal_angle,
                is_in_overworld,
                is_in_nether
            })),
            PartialPlayerPosition {
                x_in_overworld: None,
                z_in_overworld: None,
                horizontal_angle: None,
                is_in_overworld: None,
                is_in_nether: None
            } => Ok(None),
            _ => Err(D::Error::custom("player position is missing some fields"))
        }
    }
}
