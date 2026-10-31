use serde::Deserialize;

use super::MessageKind;
use super::NextThrow;
use super::Severity;

#[derive(Debug, Clone, Deserialize)]
pub struct InformationMessage {
    pub severity: Severity,
    #[serde(rename = "type")]
    pub kind: MessageKind,
    #[serde(default)]
    pub message: String
}

impl InformationMessage {
    pub fn label(&self) -> String {
        self.kind.label()
    }

    pub fn is_fatal(&self) -> bool {
        self.severity == Severity::Error
            || matches!(self.kind, MessageKind::McVersion | MessageKind::Mismeasure)
    }

    pub fn keeps_adjustment(&self) -> bool {
        self.kind == MessageKind::Mismeasure
    }

    pub fn next_throw(&self) -> Option<NextThrow> {
        if self.kind != MessageKind::NextThrowDirection {
            return None;
        }

        NextThrow::parse(&self.message)
    }
}
