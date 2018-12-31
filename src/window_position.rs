use serde_derive::Deserialize;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WindowPosition {
    Top,
    Bottom,
    Centered,
}
