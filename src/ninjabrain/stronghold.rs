use serde::Deserialize;

use super::EyeThrow;
use super::Prediction;
use super::ResultType;

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stronghold {
    pub result_type: ResultType,
    pub eye_throws: Vec<EyeThrow>,
    pub predictions: Vec<Prediction>
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
