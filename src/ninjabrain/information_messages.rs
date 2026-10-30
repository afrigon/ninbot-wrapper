use serde::Deserialize;

use super::InformationMessage;
use super::Severity;

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InformationMessages {
    pub information_messages: Vec<InformationMessage>
}

impl InformationMessages {
    pub fn warnings(&self) -> impl Iterator<Item = &InformationMessage> {
        self.information_messages
            .iter()
            .filter(|message| message.severity != Severity::Info)
    }
}
