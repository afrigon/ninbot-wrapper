use serde::Deserialize;

use super::EyeThrow;
use super::PlayerPosition;
use super::Prediction;
use super::ResultType;

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stronghold {
    pub result_type: ResultType,
    pub eye_throws: Vec<EyeThrow>,
    pub predictions: Vec<Prediction>,
    #[serde(default, deserialize_with = "PlayerPosition::deserialize_optional")]
    pub player_position: Option<PlayerPosition>
}

impl Stronghold {
    pub fn prediction(&self) -> Option<&Prediction> {
        if self.result_type != ResultType::Triangulation {
            return None;
        }

        self.predictions.first()
    }

    pub fn throws(&self) -> usize {
        self.eye_throws.len()
    }

    pub fn correction_increments(&self) -> i32 {
        self.eye_throws
            .last()
            .map_or(0, |throw| throw.correction_increments)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn reads_a_reset_without_a_player_position() {
        let reset: Stronghold = serde_json::from_value(json!({
            "eyeThrows": [],
            "resultType": "NONE",
            "playerPosition": {},
            "predictions": []
        }))
        .unwrap();

        assert_eq!(reset.result_type, ResultType::None);
        assert!(reset.player_position.is_none());
        assert_eq!(reset.throws(), 0);
    }

    #[test]
    fn reads_a_complete_player_position() {
        let measured: Stronghold = serde_json::from_value(json!({
            "eyeThrows": [],
            "resultType": "NONE",
            "playerPosition": {
                "xInOverworld": 911.8,
                "zInOverworld": -1238.09,
                "horizontalAngle": 74.34,
                "isInOverworld": true,
                "isInNether": false
            },
            "predictions": []
        }))
        .unwrap();

        assert!(measured.player_position.is_some());
    }

    #[test]
    fn rejects_a_partial_player_position() {
        let partial = serde_json::from_value::<Stronghold>(json!({
            "eyeThrows": [],
            "resultType": "NONE",
            "playerPosition": { "xInOverworld": 911.8 },
            "predictions": []
        }));

        assert!(partial.is_err());
    }
}
