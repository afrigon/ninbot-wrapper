use serde::Deserialize;

use super::InformationMessage;
use super::MessageKind;
use super::NextThrow;
use super::Severity;

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InformationMessages {
    pub information_messages: Vec<InformationMessage>
}

impl InformationMessages {
    pub fn fatal(&self) -> impl Iterator<Item = &InformationMessage> {
        self.information_messages
            .iter()
            .filter(|message| message.is_fatal())
    }

    pub fn warnings(&self) -> impl Iterator<Item = &InformationMessage> {
        self.information_messages
            .iter()
            .filter(|message| message.severity != Severity::Info && !message.is_fatal())
    }

    pub fn reports(&self, kind: &MessageKind) -> bool {
        self.information_messages
            .iter()
            .any(|message| message.kind == *kind)
    }

    pub fn next_throw(&self) -> Option<NextThrow> {
        self.information_messages
            .iter()
            .find_map(InformationMessage::next_throw)
    }
}
