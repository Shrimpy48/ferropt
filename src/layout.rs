use enum_map::Enum;
use serde::{Deserialize, Serialize};

use std::fmt;
use std::mem::{discriminant, Discriminant};
use std::ops::{Index, IndexMut};
use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

pub const NUM_KEYS: usize = 34;
pub const NUM_LAYERS: usize = 3;

lazy_static! {
    static ref KC_RE: Regex =
        Regex::new(r"^LSFT\((?P<shifted>[^()]+)\)|OSL\((?P<layer>\d+)\)$",).unwrap();
}

pub const HOMING: [KeyCode; 6] = [
    KeyCode::F,
    KeyCode::J,
    KeyCode::T,
    KeyCode::N,
    KeyCode::U,
    KeyCode::H,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyCode {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Digit0,
    Digit1,
    Digit2,
    Digit3,
    Digit4,
    Digit5,
    Digit6,
    Digit7,
    Digit8,
    Digit9,
    Comma,
    Dot,
    Apostrophe,
    Semicolon,
    Backslash,
    Slash,
    LeftSquareBracket,
    RightSquareBracket,
    Hash,
    Grave,
    Minus,
    Equals,
    Space,
}

impl KeyCode {
    fn typed_char(self) -> char {
        match self {
            Self::A => 'a',
            Self::B => 'b',
            Self::C => 'c',
            Self::D => 'd',
            Self::E => 'e',
            Self::F => 'f',
            Self::G => 'g',
            Self::H => 'h',
            Self::I => 'i',
            Self::J => 'j',
            Self::K => 'k',
            Self::L => 'l',
            Self::M => 'm',
            Self::N => 'n',
            Self::O => 'o',
            Self::P => 'p',
            Self::Q => 'q',
            Self::R => 'r',
            Self::S => 's',
            Self::T => 't',
            Self::U => 'u',
            Self::V => 'v',
            Self::W => 'w',
            Self::X => 'x',
            Self::Y => 'y',
            Self::Z => 'z',
            Self::Digit0 => '0',
            Self::Digit1 => '1',
            Self::Digit2 => '2',
            Self::Digit3 => '3',
            Self::Digit4 => '4',
            Self::Digit5 => '5',
            Self::Digit6 => '6',
            Self::Digit7 => '7',
            Self::Digit8 => '8',
            Self::Digit9 => '9',
            Self::Comma => ',',
            Self::Dot => '.',
            Self::Apostrophe => '\'',
            Self::Semicolon => ';',
            Self::Backslash => '\\',
            Self::Slash => '/',
            Self::LeftSquareBracket => '[',
            Self::RightSquareBracket => ']',
            Self::Hash => '#',
            Self::Grave => '`',
            Self::Minus => '-',
            Self::Equals => '=',
            Self::Space => ' ',
        }
    }

    fn shifted_char(self) -> char {
        match self {
            Self::A => 'A',
            Self::B => 'B',
            Self::C => 'C',
            Self::D => 'D',
            Self::E => 'E',
            Self::F => 'F',
            Self::G => 'G',
            Self::H => 'H',
            Self::I => 'I',
            Self::J => 'J',
            Self::K => 'K',
            Self::L => 'L',
            Self::M => 'M',
            Self::N => 'N',
            Self::O => 'O',
            Self::P => 'P',
            Self::Q => 'Q',
            Self::R => 'R',
            Self::S => 'S',
            Self::T => 'T',
            Self::U => 'U',
            Self::V => 'V',
            Self::W => 'W',
            Self::X => 'X',
            Self::Y => 'Y',
            Self::Z => 'Z',
            Self::Digit0 => ')',
            Self::Digit1 => '!',
            Self::Digit2 => '"',
            Self::Digit3 => '£',
            Self::Digit4 => '$',
            Self::Digit5 => '%',
            Self::Digit6 => '^',
            Self::Digit7 => '&',
            Self::Digit8 => '*',
            Self::Digit9 => '(',
            Self::Comma => '<',
            Self::Dot => '>',
            Self::Apostrophe => '@',
            Self::Semicolon => ':',
            Self::Backslash => '|',
            Self::Slash => '?',
            Self::LeftSquareBracket => '{',
            Self::RightSquareBracket => '}',
            Self::Hash => '~',
            Self::Grave => '¬',
            Self::Minus => '_',
            Self::Equals => '+',
            Self::Space => ' ',
        }
    }
}

impl fmt::Display for KeyCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::A => write!(f, "KC_A"),
            Self::B => write!(f, "KC_B"),
            Self::C => write!(f, "KC_C"),
            Self::D => write!(f, "KC_D"),
            Self::E => write!(f, "KC_E"),
            Self::F => write!(f, "KC_F"),
            Self::G => write!(f, "KC_G"),
            Self::H => write!(f, "KC_H"),
            Self::I => write!(f, "KC_I"),
            Self::J => write!(f, "KC_J"),
            Self::K => write!(f, "KC_K"),
            Self::L => write!(f, "KC_L"),
            Self::M => write!(f, "KC_M"),
            Self::N => write!(f, "KC_N"),
            Self::O => write!(f, "KC_O"),
            Self::P => write!(f, "KC_P"),
            Self::Q => write!(f, "KC_Q"),
            Self::R => write!(f, "KC_R"),
            Self::S => write!(f, "KC_S"),
            Self::T => write!(f, "KC_T"),
            Self::U => write!(f, "KC_U"),
            Self::V => write!(f, "KC_V"),
            Self::W => write!(f, "KC_W"),
            Self::X => write!(f, "KC_X"),
            Self::Y => write!(f, "KC_Y"),
            Self::Z => write!(f, "KC_Z"),
            Self::Digit0 => write!(f, "KC_0"),
            Self::Digit1 => write!(f, "KC_1"),
            Self::Digit2 => write!(f, "KC_2"),
            Self::Digit3 => write!(f, "KC_3"),
            Self::Digit4 => write!(f, "KC_4"),
            Self::Digit5 => write!(f, "KC_5"),
            Self::Digit6 => write!(f, "KC_6"),
            Self::Digit7 => write!(f, "KC_7"),
            Self::Digit8 => write!(f, "KC_8"),
            Self::Digit9 => write!(f, "KC_9"),
            Self::Comma => write!(f, "KC_COMM"),
            Self::Dot => write!(f, "KC_DOT"),
            Self::Apostrophe => write!(f, "KC_QUOT"),
            Self::Semicolon => write!(f, "KC_SCLN"),
            Self::Backslash => write!(f, "KC_NUBS"),
            Self::Slash => write!(f, "KC_SLSH"),
            Self::LeftSquareBracket => write!(f, "KC_LBRC"),
            Self::RightSquareBracket => write!(f, "KC_RBRC"),
            Self::Hash => write!(f, "KC_NUHS"),
            Self::Grave => write!(f, "KC_GRV"),
            Self::Minus => write!(f, "KC_MINS"),
            Self::Equals => write!(f, "KC_EQL"),
            Self::Space => write!(f, "KC_SPC"),
        }
    }
}

impl FromStr for KeyCode {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "KC_A" => Ok(Self::A),
            "KC_B" => Ok(Self::B),
            "KC_C" => Ok(Self::C),
            "KC_D" => Ok(Self::D),
            "KC_E" => Ok(Self::E),
            "KC_F" => Ok(Self::F),
            "KC_G" => Ok(Self::G),
            "KC_H" => Ok(Self::H),
            "KC_I" => Ok(Self::I),
            "KC_J" => Ok(Self::J),
            "KC_K" => Ok(Self::K),
            "KC_L" => Ok(Self::L),
            "KC_M" => Ok(Self::M),
            "KC_N" => Ok(Self::N),
            "KC_O" => Ok(Self::O),
            "KC_P" => Ok(Self::P),
            "KC_Q" => Ok(Self::Q),
            "KC_R" => Ok(Self::R),
            "KC_S" => Ok(Self::S),
            "KC_T" => Ok(Self::T),
            "KC_U" => Ok(Self::U),
            "KC_V" => Ok(Self::V),
            "KC_W" => Ok(Self::W),
            "KC_X" => Ok(Self::X),
            "KC_Y" => Ok(Self::Y),
            "KC_Z" => Ok(Self::Z),
            "KC_0" => Ok(Self::Digit0),
            "KC_1" => Ok(Self::Digit1),
            "KC_2" => Ok(Self::Digit2),
            "KC_3" => Ok(Self::Digit3),
            "KC_4" => Ok(Self::Digit4),
            "KC_5" => Ok(Self::Digit5),
            "KC_6" => Ok(Self::Digit6),
            "KC_7" => Ok(Self::Digit7),
            "KC_8" => Ok(Self::Digit8),
            "KC_9" => Ok(Self::Digit9),
            "KC_COMM" => Ok(Self::Comma),
            "KC_DOT" => Ok(Self::Dot),
            "KC_QUOT" => Ok(Self::Apostrophe),
            "KC_SCLN" => Ok(Self::Semicolon),
            "KC_NUBS" => Ok(Self::Backslash),
            "KC_SLSH" => Ok(Self::Slash),
            "KC_LBRC" => Ok(Self::LeftSquareBracket),
            "KC_RBRC" => Ok(Self::RightSquareBracket),
            "KC_NUHS" => Ok(Self::Hash),
            "KC_GRV" => Ok(Self::Grave),
            "KC_MINS" => Ok(Self::Minus),
            "KC_EQL" => Ok(Self::Equals),
            "KC_SPC" => Ok(Self::Space),
            _ => Err(ParseError::UnknownValue(s.to_string())),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    Typing(KeyCode),
    Shifted(KeyCode),
    Empty,
    Shift,
    Layer(usize),
    // Repeat,
}

impl Key {
    pub fn typed_char(self, shifted: bool) -> Option<char> {
        match self {
            Self::Typing(kc) => Some(if shifted {
                kc.shifted_char()
            } else {
                kc.typed_char()
            }),
            Self::Shifted(kc) => Some(kc.shifted_char()),
            _ => None,
        }
    }
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Typing(kc) => write!(f, "{}", kc),
            Self::Shifted(kc) => write!(f, "LSFT({})", kc),
            Self::Empty => write!(f, "KC_NO"),
            Self::Shift => write!(f, "OSM(MOD_LSFT)"),
            Self::Layer(i) => write!(f, "OSL({})", i),
        }
    }
}

#[derive(Debug)]
pub enum ParseError {
    UnknownValue(String),
    MissingValue(String),
    WrongType {
        expected: Discriminant<serde_json::Value>,
        found: Discriminant<serde_json::Value>,
    },
    WrongLength {
        expected: usize,
        found: usize,
    },
    WrongValue {
        expected: String,
        found: String,
    },
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownValue(s) => write!(f, "unknown value: {}", s),
            Self::MissingValue(s) => write!(f, "missing value: {}", s),
            Self::WrongValue { expected, found } => write!(
                f,
                "incorrect value (expected {}, found {})",
                expected, found
            ),
            Self::WrongType { expected, found } => write!(
                f,
                "incorrect type (expected {:?}, found {:?})",
                expected, found
            ),
            Self::WrongLength { expected, found } => write!(
                f,
                "incorrect length (expected {}, found {})",
                expected, found
            ),
        }
    }
}

impl std::error::Error for ParseError {}

impl FromStr for Key {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "KC_NO" => Ok(Self::Empty),
            "OSM(MOD_LSFT)" => Ok(Self::Shift),
            s => {
                if let Ok(kc) = s.parse() {
                    return Ok(Self::Typing(kc));
                }
                let cap = KC_RE
                    .captures(s)
                    .ok_or_else(|| ParseError::UnknownValue(s.to_string()))?;
                if let Some(kc_name) = cap.name("shifted") {
                    let kc = kc_name.as_str().parse()?;
                    Ok(Self::Shifted(kc))
                } else if let Some(c) = cap.name("layer") {
                    let i = c.as_str().parse().unwrap();
                    Ok(Self::Layer(i))
                } else {
                    Err(ParseError::UnknownValue(s.to_string()))
                }
            }
        }
    }
}

impl From<Key> for serde_json::Value {
    fn from(k: Key) -> Self {
        serde_json::Value::String(k.to_string())
    }
}

impl TryFrom<serde_json::Value> for Key {
    type Error = ParseError;

    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        if let serde_json::Value::String(s) = value {
            s.parse()
        } else {
            Err(ParseError::WrongType {
                expected: discriminant(&serde_json::Value::String(String::new())),
                found: discriminant(&value),
            })
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct Layer<T>(pub(crate) [T; NUM_KEYS]);

impl<T> Layer<T> {
    pub(crate) fn iter(&self) -> impl Iterator<Item = &T> {
        self.0.iter()
    }
}

impl<T: Into<serde_json::Value>> From<Layer<T>> for serde_json::Value {
    fn from(l: Layer<T>) -> Self {
        serde_json::Value::Array(l.0.into_iter().map(|v| v.into()).collect())
    }
}

impl<T: TryFrom<serde_json::Value, Error = ParseError>> TryFrom<serde_json::Value> for Layer<T> {
    type Error = ParseError;

    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        match value {
            serde_json::Value::Array(v) => v
                .into_iter()
                .map(|x| x.try_into())
                .collect::<Result<Vec<_>, _>>()
                .and_then(|v| {
                    v.try_into().map_err(|v: Vec<_>| ParseError::WrongLength {
                        expected: NUM_KEYS,
                        found: v.len(),
                    })
                })
                .map(Self),
            _ => Err(ParseError::WrongType {
                expected: discriminant(&serde_json::Value::Array(Vec::new())),
                found: discriminant(&value),
            }),
        }
    }
}

impl<T> Index<usize> for Layer<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T> IndexMut<usize> for Layer<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "serde_json::Value")]
#[serde(into = "serde_json::Value")]
#[repr(transparent)]
pub struct Layout {
    pub(crate) layers: [Layer<Key>; NUM_LAYERS],
}

impl Layout {
    pub(crate) fn iter(&self) -> impl Iterator<Item = &Layer<Key>> {
        self.layers.iter()
    }

    fn has_key(&self, key: Key) -> bool {
        self.iter().any(|l| l.iter().any(|&k| k == key))
    }

    pub(crate) fn hamming_dist(&self, other: &Self) -> u8 {
        let self_keys = self.iter().flat_map(|l| l.iter());
        let other_keys = other.iter().flat_map(|l| l.iter());
        self_keys.zip(other_keys).filter(|(a, b)| a != b).count() as u8
    }
}

impl From<Layout> for serde_json::Value {
    fn from(l: Layout) -> Self {
        let mut map = serde_json::Map::new();
        map.insert("keyboard".into(), "ferris/sweep".into());
        map.insert("keymap".into(), "ferropt".into());
        map.insert("layout".into(), "LAYOUT_split_3x5_2".into());
        map.insert(
            "layers".into(),
            serde_json::Value::Array(l.layers.into_iter().map(|v| v.into()).collect()),
        );
        serde_json::Value::Object(map)
    }
}

impl TryFrom<serde_json::Value> for Layout {
    type Error = ParseError;

    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        match value {
            serde_json::Value::Object(map) => {
                if !map
                    .get("keyboard")
                    .map(|s| s == "ferris/sweep")
                    .unwrap_or(false)
                {
                    return Err(ParseError::MissingValue("keyboard".into()));
                }
                if !map
                    .get("layout")
                    .map(|s| s == "LAYOUT_split_3x5_2")
                    .unwrap_or(false)
                {
                    return Err(ParseError::MissingValue("layout".into()));
                }
                let layers = map
                    .get("layers")
                    .ok_or_else(|| ParseError::MissingValue("layers".into()))
                    .and_then(|ls| match ls {
                        serde_json::Value::Array(v) => v
                            .iter()
                            .map(|x| x.clone().try_into())
                            .collect::<Result<Vec<_>, _>>()
                            .and_then(|v| {
                                v.try_into().map_err(|v: Vec<_>| ParseError::WrongLength {
                                    expected: NUM_LAYERS,
                                    found: v.len(),
                                })
                            }),
                        _ => Err(ParseError::WrongType {
                            expected: discriminant(&serde_json::Value::Array(Vec::new())),
                            found: discriminant(ls),
                        }),
                    })?;
                Ok(Self { layers })
            }
            _ => Err(ParseError::WrongType {
                expected: discriminant(&serde_json::Value::Object(serde_json::Map::new())),
                found: discriminant(&value),
            }),
        }
    }
}

impl Index<usize> for Layout {
    type Output = Layer<Key>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.layers[index]
    }
}

impl IndexMut<usize> for Layout {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.layers[index]
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Enum)]
pub enum Finger {
    Pinky,
    Ring,
    Middle,
    Index,
    Thumb,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Enum)]
pub enum Hand {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Enum)]
pub struct Digit {
    pub hand: Hand,
    pub finger: Finger,
}

pub fn finger_for_pos(row: usize, col: usize) -> Digit {
    if row == 3 {
        match col {
            0 | 1 => Digit {
                hand: Hand::Left,
                finger: Finger::Thumb,
            },
            2 | 3 => Digit {
                hand: Hand::Right,
                finger: Finger::Thumb,
            },
            _ => panic!("invalid column {} for row {}", col, row),
        }
    } else {
        match col {
            0 => Digit {
                hand: Hand::Left,
                finger: Finger::Pinky,
            },
            1 => Digit {
                hand: Hand::Left,
                finger: Finger::Ring,
            },
            2 => Digit {
                hand: Hand::Left,
                finger: Finger::Middle,
            },
            3 | 4 => Digit {
                hand: Hand::Left,
                finger: Finger::Index,
            },
            5 | 6 => Digit {
                hand: Hand::Right,
                finger: Finger::Index,
            },
            7 => Digit {
                hand: Hand::Right,
                finger: Finger::Middle,
            },
            8 => Digit {
                hand: Hand::Right,
                finger: Finger::Ring,
            },
            9 => Digit {
                hand: Hand::Right,
                finger: Finger::Pinky,
            },
            _ => panic!("invalid column {} for row {}", col, row),
        }
    }
}
