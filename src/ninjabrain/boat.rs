use serde::Deserialize;

use super::BoatState;

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Boat {
    pub boat_state: BoatState
}
