use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(from = "String")]
pub enum MessageKind {
    Mismeasure,
    PortalLinking,
    McVersion,
    CombinedCertainty,
    NextThrowDirection,
    Other(String)
}

impl MessageKind {
    pub fn label(&self) -> String {
        match self {
            Self::Mismeasure => "mismeasure".to_string(),
            Self::PortalLinking => "portal linking".to_string(),
            Self::McVersion => "wrong minecraft version".to_string(),
            Self::CombinedCertainty => "combined certainty".to_string(),
            Self::NextThrowDirection => "next throw direction".to_string(),
            Self::Other(kind) => kind.to_lowercase().replace('_', " ")
        }
    }
}

impl From<String> for MessageKind {
    fn from(kind: String) -> Self {
        match kind.as_str() {
            "MISMEASURE" => Self::Mismeasure,
            "PORTAL_LINKING" => Self::PortalLinking,
            "MC_VERSION" => Self::McVersion,
            "COMBINED_CERTAINTY" => Self::CombinedCertainty,
            "NEXT_THROW_DIRECTION" => Self::NextThrowDirection,
            _ => Self::Other(kind)
        }
    }
}
