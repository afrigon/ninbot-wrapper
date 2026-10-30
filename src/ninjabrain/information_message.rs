use serde::Deserialize;

use super::Severity;

#[derive(Debug, Clone, Deserialize)]
pub struct InformationMessage {
    pub severity: Severity,
    #[serde(rename = "type")]
    pub kind: String
}

impl InformationMessage {
    pub fn label(&self) -> String {
        self.kind.to_lowercase().replace('_', " ")
    }
}
