use serde::Deserialize;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ResultType {
    #[default]
    None,
    Triangulation,
    Failed,
    #[serde(other)]
    Other
}
