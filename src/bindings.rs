use std::fmt::{self, Display};
use std::str::FromStr;

use conrod::backend::glium::glium::glutin::{ModifiersState, VirtualKeyCode};
use serde::de;

/**
 * A pressed key
 */
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyCode(VirtualKeyCode);

impl FromStr for KeyCode {
    type Err = ShortcutFromStrError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "a" => Ok(KeyCode(VirtualKeyCode::A)),
            "b" => Ok(KeyCode(VirtualKeyCode::B)),
            "c" => Ok(KeyCode(VirtualKeyCode::C)),
            "d" => Ok(KeyCode(VirtualKeyCode::D)),
            "e" => Ok(KeyCode(VirtualKeyCode::E)),
            "f" => Ok(KeyCode(VirtualKeyCode::F)),
            "g" => Ok(KeyCode(VirtualKeyCode::G)),
            "h" => Ok(KeyCode(VirtualKeyCode::H)),
            "i" => Ok(KeyCode(VirtualKeyCode::I)),
            "j" => Ok(KeyCode(VirtualKeyCode::J)),
            "k" => Ok(KeyCode(VirtualKeyCode::K)),
            "l" => Ok(KeyCode(VirtualKeyCode::L)),
            "m" => Ok(KeyCode(VirtualKeyCode::M)),
            "n" => Ok(KeyCode(VirtualKeyCode::N)),
            "o" => Ok(KeyCode(VirtualKeyCode::O)),
            "p" => Ok(KeyCode(VirtualKeyCode::P)),
            "q" => Ok(KeyCode(VirtualKeyCode::Q)),
            "r" => Ok(KeyCode(VirtualKeyCode::R)),
            "s" => Ok(KeyCode(VirtualKeyCode::S)),
            "t" => Ok(KeyCode(VirtualKeyCode::T)),
            "u" => Ok(KeyCode(VirtualKeyCode::U)),
            "v" => Ok(KeyCode(VirtualKeyCode::V)),
            "w" => Ok(KeyCode(VirtualKeyCode::W)),
            "x" => Ok(KeyCode(VirtualKeyCode::X)),
            "y" => Ok(KeyCode(VirtualKeyCode::Y)),
            "z" => Ok(KeyCode(VirtualKeyCode::Z)),
            "1" => Ok(KeyCode(VirtualKeyCode::Key1)),
            "2" => Ok(KeyCode(VirtualKeyCode::Key2)),
            "3" => Ok(KeyCode(VirtualKeyCode::Key3)),
            "4" => Ok(KeyCode(VirtualKeyCode::Key4)),
            "5" => Ok(KeyCode(VirtualKeyCode::Key5)),
            "6" => Ok(KeyCode(VirtualKeyCode::Key6)),
            "7" => Ok(KeyCode(VirtualKeyCode::Key7)),
            "8" => Ok(KeyCode(VirtualKeyCode::Key8)),
            "9" => Ok(KeyCode(VirtualKeyCode::Key9)),
            "0" => Ok(KeyCode(VirtualKeyCode::Key0)),
            "SPC" => Ok(KeyCode(VirtualKeyCode::Space)),
            _ => Err(ShortcutFromStrError),
        }
    }
}

impl From<VirtualKeyCode> for KeyCode {
    fn from(value: VirtualKeyCode) -> Self {
        KeyCode(value)
    }
}

impl Display for KeyCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let keyname = match self.0 {
            VirtualKeyCode::A => Some("a"),
            VirtualKeyCode::B => Some("b"),
            VirtualKeyCode::C => Some("c"),
            VirtualKeyCode::D => Some("d"),
            VirtualKeyCode::E => Some("e"),
            VirtualKeyCode::F => Some("f"),
            VirtualKeyCode::G => Some("g"),
            VirtualKeyCode::H => Some("h"),
            VirtualKeyCode::I => Some("i"),
            VirtualKeyCode::J => Some("j"),
            VirtualKeyCode::K => Some("k"),
            VirtualKeyCode::L => Some("l"),
            VirtualKeyCode::M => Some("m"),
            VirtualKeyCode::N => Some("n"),
            VirtualKeyCode::O => Some("o"),
            VirtualKeyCode::P => Some("p"),
            VirtualKeyCode::Q => Some("q"),
            VirtualKeyCode::R => Some("r"),
            VirtualKeyCode::S => Some("s"),
            VirtualKeyCode::T => Some("t"),
            VirtualKeyCode::U => Some("u"),
            VirtualKeyCode::V => Some("v"),
            VirtualKeyCode::W => Some("w"),
            VirtualKeyCode::X => Some("x"),
            VirtualKeyCode::Y => Some("y"),
            VirtualKeyCode::Z => Some("z"),
            VirtualKeyCode::Key1 => Some("1"),
            VirtualKeyCode::Key2 => Some("2"),
            VirtualKeyCode::Key3 => Some("3"),
            VirtualKeyCode::Key4 => Some("4"),
            VirtualKeyCode::Key5 => Some("5"),
            VirtualKeyCode::Key6 => Some("6"),
            VirtualKeyCode::Key7 => Some("7"),
            VirtualKeyCode::Key8 => Some("8"),
            VirtualKeyCode::Key9 => Some("9"),
            VirtualKeyCode::Key0 => Some("0"),
            _ => None,
        };

        match keyname {
            Some(name) => write!(f, "{}", name),
            None => write!(f, "<unknown>"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Shortcut {
    pub key_code: KeyCode,
    pub modifiers: ModifiersState,
}

impl Display for Shortcut {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.modifiers.ctrl {
            write!(f, "C-")?;
        }
        if self.modifiers.alt {
            write!(f, "M-")?;
        }
        if self.modifiers.shift {
            write!(f, "S-")?;
        }
        if self.modifiers.logo {
            write!(f, "L-")?;
        }
        write!(f, "{}", self.key_code)
    }
}

impl FromStr for Shortcut {
    type Err = ShortcutFromStrError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let keys: Vec<&str> = value.split('-').collect::<Vec<&str>>();;
        if let Some((key_code_string, modifier_strings)) = keys.split_last() {
            let modifiers = ModifiersState {
                ctrl: modifier_strings.contains(&"C"),
                alt: modifier_strings.contains(&"M"),
                shift: modifier_strings.contains(&"S"),
                logo: modifier_strings.contains(&"L"),
            };
            let key_code = key_code_string.parse()?;
            Ok(Shortcut {
                modifiers,
                key_code
            })
        } else {
            Err(ShortcutFromStrError)
        }
    }
}

impl<'de> de::Deserialize<'de> for Shortcut {
    fn deserialize<D>(deserializer: D) -> Result<Shortcut, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let s = <&str>::deserialize(deserializer)?;
        FromStr::from_str(s).map_err(de::Error::custom)
    }
}

#[derive(Copy, Clone)]
pub struct ShortcutFromStrError;

impl Display for ShortcutFromStrError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Not a valid shortcut identifier")
    }
}

impl fmt::Debug for ShortcutFromStrError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Not a valid shortcut identifier")
    }
}
