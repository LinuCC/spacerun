use serde::de;
use std::fmt;

#[derive(Clone, Debug)]
pub enum WindowPosition {
    Top,
    Bottom,
    Centered,
}

struct WindowPositionVisitor;

impl<'de> de::Visitor<'de> for WindowPositionVisitor {
    type Value = WindowPosition;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a window position")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match value {
            "top" => Ok(WindowPosition::Top),
            "centered" => Ok(WindowPosition::Centered),
            "bottom" => Ok(WindowPosition::Bottom),
            _ => Err(de::Error::custom(format!(
                "Not a parseable window position: {}",
                value
            ))),
        }
    }
}

impl<'de> de::Deserialize<'de> for WindowPosition {
    fn deserialize<D>(deserializer: D) -> Result<WindowPosition, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_str(WindowPositionVisitor)
    }
}
