use serde::Deserialize;

use super::Dimension;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerPosition {
    pub x_in_overworld: f64,
    pub z_in_overworld: f64,
    pub horizontal_angle: f64,
    pub is_in_overworld: bool,
    pub is_in_nether: bool
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
}
