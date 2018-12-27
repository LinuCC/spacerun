use conrod::backend::glium::glium;
use conrod::backend::glium::glium::glutin::{ModifiersState, VirtualKeyCode};
use serde::de;
use std::fmt::{self, Display};

/**
 * A pressed key
 */
#[derive(Debug, Clone)]
pub struct KeyCode(VirtualKeyCode);

fn key_code_from_str<E>(value: &str) -> Result<KeyCode, E>
where
    E: de::Error,
{
    match value {
        "a" => Ok(KeyCode(glium::glutin::VirtualKeyCode::A)),
        "b" => Ok(KeyCode(glium::glutin::VirtualKeyCode::B)),
        "c" => Ok(KeyCode(glium::glutin::VirtualKeyCode::C)),
        "d" => Ok(KeyCode(glium::glutin::VirtualKeyCode::D)),
        "e" => Ok(KeyCode(glium::glutin::VirtualKeyCode::E)),
        "f" => Ok(KeyCode(glium::glutin::VirtualKeyCode::F)),
        "g" => Ok(KeyCode(glium::glutin::VirtualKeyCode::G)),
        "h" => Ok(KeyCode(glium::glutin::VirtualKeyCode::H)),
        "i" => Ok(KeyCode(glium::glutin::VirtualKeyCode::I)),
        "j" => Ok(KeyCode(glium::glutin::VirtualKeyCode::J)),
        "k" => Ok(KeyCode(glium::glutin::VirtualKeyCode::K)),
        "l" => Ok(KeyCode(glium::glutin::VirtualKeyCode::L)),
        "m" => Ok(KeyCode(glium::glutin::VirtualKeyCode::M)),
        "n" => Ok(KeyCode(glium::glutin::VirtualKeyCode::N)),
        "o" => Ok(KeyCode(glium::glutin::VirtualKeyCode::O)),
        "p" => Ok(KeyCode(glium::glutin::VirtualKeyCode::P)),
        "q" => Ok(KeyCode(glium::glutin::VirtualKeyCode::Q)),
        "r" => Ok(KeyCode(glium::glutin::VirtualKeyCode::R)),
        "s" => Ok(KeyCode(glium::glutin::VirtualKeyCode::S)),
        "t" => Ok(KeyCode(glium::glutin::VirtualKeyCode::T)),
        "u" => Ok(KeyCode(glium::glutin::VirtualKeyCode::U)),
        "v" => Ok(KeyCode(glium::glutin::VirtualKeyCode::V)),
        "w" => Ok(KeyCode(glium::glutin::VirtualKeyCode::W)),
        "x" => Ok(KeyCode(glium::glutin::VirtualKeyCode::X)),
        "y" => Ok(KeyCode(glium::glutin::VirtualKeyCode::Y)),
        "z" => Ok(KeyCode(glium::glutin::VirtualKeyCode::Z)),
        "1" => Ok(KeyCode(glium::glutin::VirtualKeyCode::Key1)),
        "2" => Ok(KeyCode(glium::glutin::VirtualKeyCode::Key2)),
        "3" => Ok(KeyCode(glium::glutin::VirtualKeyCode::Key3)),
        "4" => Ok(KeyCode(glium::glutin::VirtualKeyCode::Key4)),
        "5" => Ok(KeyCode(glium::glutin::VirtualKeyCode::Key5)),
        "6" => Ok(KeyCode(glium::glutin::VirtualKeyCode::Key6)),
        "7" => Ok(KeyCode(glium::glutin::VirtualKeyCode::Key7)),
        "8" => Ok(KeyCode(glium::glutin::VirtualKeyCode::Key8)),
        "9" => Ok(KeyCode(glium::glutin::VirtualKeyCode::Key9)),
        "0" => Ok(KeyCode(glium::glutin::VirtualKeyCode::Key0)),
        _ => Err(de::Error::custom(format!(
            "Not a parseable shortcut identifier: {}",
            value
        ))),
    }
}

struct KeyCodeVisitor;

impl<'de> de::Visitor<'de> for KeyCodeVisitor {
    type Value = KeyCode;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a shortcut identifier")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        key_code_from_str(value)
    }
}

impl<'de> de::Deserialize<'de> for KeyCode {
    fn deserialize<D>(deserializer: D) -> Result<KeyCode, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_str(KeyCodeVisitor)
    }
}

impl PartialEq<glium::glutin::VirtualKeyCode> for KeyCode {
    fn eq(&self, rhs: &glium::glutin::VirtualKeyCode) -> bool {
        self.0 == *rhs
    }
}

impl Display for KeyCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let keyname = match self.0 {
            glium::glutin::VirtualKeyCode::A => Some("a"),
            glium::glutin::VirtualKeyCode::B => Some("b"),
            glium::glutin::VirtualKeyCode::C => Some("c"),
            glium::glutin::VirtualKeyCode::D => Some("d"),
            glium::glutin::VirtualKeyCode::E => Some("e"),
            glium::glutin::VirtualKeyCode::F => Some("f"),
            glium::glutin::VirtualKeyCode::G => Some("g"),
            glium::glutin::VirtualKeyCode::H => Some("h"),
            glium::glutin::VirtualKeyCode::I => Some("i"),
            glium::glutin::VirtualKeyCode::J => Some("j"),
            glium::glutin::VirtualKeyCode::K => Some("k"),
            glium::glutin::VirtualKeyCode::L => Some("l"),
            glium::glutin::VirtualKeyCode::M => Some("m"),
            glium::glutin::VirtualKeyCode::N => Some("n"),
            glium::glutin::VirtualKeyCode::O => Some("o"),
            glium::glutin::VirtualKeyCode::P => Some("p"),
            glium::glutin::VirtualKeyCode::Q => Some("q"),
            glium::glutin::VirtualKeyCode::R => Some("r"),
            glium::glutin::VirtualKeyCode::S => Some("s"),
            glium::glutin::VirtualKeyCode::T => Some("t"),
            glium::glutin::VirtualKeyCode::U => Some("u"),
            glium::glutin::VirtualKeyCode::V => Some("v"),
            glium::glutin::VirtualKeyCode::W => Some("w"),
            glium::glutin::VirtualKeyCode::X => Some("x"),
            glium::glutin::VirtualKeyCode::Y => Some("y"),
            glium::glutin::VirtualKeyCode::Z => Some("z"),
            glium::glutin::VirtualKeyCode::Key1 => Some("1"),
            glium::glutin::VirtualKeyCode::Key2 => Some("2"),
            glium::glutin::VirtualKeyCode::Key3 => Some("3"),
            glium::glutin::VirtualKeyCode::Key4 => Some("4"),
            glium::glutin::VirtualKeyCode::Key5 => Some("5"),
            glium::glutin::VirtualKeyCode::Key6 => Some("6"),
            glium::glutin::VirtualKeyCode::Key7 => Some("7"),
            glium::glutin::VirtualKeyCode::Key8 => Some("8"),
            glium::glutin::VirtualKeyCode::Key9 => Some("9"),
            glium::glutin::VirtualKeyCode::Key0 => Some("0"),
            _ => None,
        };

        match keyname {
            Some(name) => write!(f, "{}", name),
            None => write!(f, "<unknown>"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Shortcut {
    pub key_code: KeyCode,
    pub modifiers: ModifiersState,
}

impl Display for Shortcut {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = &mut String::new();
        if self.modifiers.ctrl {
            string.push_str("ctrl+");
        }
        if self.modifiers.alt {
            string.push_str("alt+");
        }
        if self.modifiers.shift {
            string.push_str("shift+");
        }
        if self.modifiers.logo {
            string.push_str("super+");
        }
        string.push_str(&format!("{}", self.key_code));
        write!(f, "{}", string)
    }
}

impl PartialEq<(&ModifiersState, &VirtualKeyCode)> for Shortcut {
    fn eq(&self, rhs: &(&ModifiersState, &VirtualKeyCode)) -> bool {
        self.key_code == *rhs.1
            && self.modifiers.ctrl == rhs.0.ctrl
            && self.modifiers.alt == rhs.0.alt
            && self.modifiers.shift == rhs.0.shift
            && self.modifiers.logo == rhs.0.logo
    }
}

struct ShortcutVisitor;

impl<'de> de::Visitor<'de> for ShortcutVisitor {
    type Value = Shortcut;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a shortcut identifier")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let keys: Vec<&str> = value.split('+').collect::<Vec<&str>>();;
        if let Some((key_code_string, modifier_strings)) = keys.split_last() {
            let modifiers_state = ModifiersState {
                ctrl: modifier_strings.contains(&"ctrl"),
                alt: modifier_strings.contains(&"alt"),
                shift: modifier_strings.contains(&"shift"),
                logo: modifier_strings.contains(&"super"),
            };
            let key_code_result = key_code_from_str(key_code_string);
            match key_code_result {
                Ok(key_code) => Ok(Shortcut {
                    modifiers: modifiers_state,
                    key_code: key_code,
                }),
                Err(error) => Err(error),
            }
        } else {
            Err(de::Error::custom(format!(
                "Not a parseable shortcut identifier: {}",
                value
            )))
        }
    }
}

impl<'de> de::Deserialize<'de> for Shortcut {
    fn deserialize<D>(deserializer: D) -> Result<Shortcut, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_str(ShortcutVisitor)
    }
}
