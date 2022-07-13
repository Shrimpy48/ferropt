use serde::{Deserialize, Serialize};

use std::fmt;
use std::mem::{discriminant, Discriminant};
use std::ops::{Index, IndexMut};
use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

pub const NUM_KEYS: usize = 34;
pub const NUM_LAYERS: usize = 4;

lazy_static! {
    static ref KC_RE: Regex =
        Regex::new(r"^LSFT\(KC_(?P<upper>[A-Z])\)|KC_(?P<lower>[A-Z0-9])|OSL\((?P<layer>\d+)\)$",)
            .unwrap();
}

// pub const DEFAULT_LAYOUT: Layout = Layout {
//     layers: [
//         Layer([
//             Key::Char('q'),
//             Key::Char('w'),
//             Key::Char('f'),
//             Key::Char('p'),
//             Key::Char('b'),
//             Key::Char('j'),
//             Key::Char('l'),
//             Key::Char('u'),
//             Key::Char('y'),
//             Key::Char(';'),
//             Key::Char('a'),
//             Key::Char('r'),
//             Key::Char('s'),
//             Key::Char('t'),
//             Key::Char('g'),
//             Key::Char('m'),
//             Key::Char('n'),
//             Key::Char('e'),
//             Key::Char('i'),
//             Key::Char('o'),
//             Key::Char('z'),
//             Key::Char('x'),
//             Key::Char('c'),
//             Key::Char('d'),
//             Key::Char('v'),
//             Key::Char('k'),
//             Key::Char('h'),
//             Key::Char(','),
//             Key::Char('.'),
//             Key::Char('/'),
//             Key::Layer(1),
//             Key::Char(' '),
//             Key::Layer(2),
//             Key::Layer(3),
//         ]),
//         Layer([
//             Key::Char('Q'),
//             Key::Char('W'),
//             Key::Char('F'),
//             Key::Char('P'),
//             Key::Char('B'),
//             Key::Char('J'),
//             Key::Char('L'),
//             Key::Char('U'),
//             Key::Char('Y'),
//             Key::Char(':'),
//             Key::Char('A'),
//             Key::Char('R'),
//             Key::Char('S'),
//             Key::Char('T'),
//             Key::Char('G'),
//             Key::Char('M'),
//             Key::Char('N'),
//             Key::Char('E'),
//             Key::Char('I'),
//             Key::Char('O'),
//             Key::Char('Z'),
//             Key::Char('X'),
//             Key::Char('C'),
//             Key::Char('D'),
//             Key::Char('V'),
//             Key::Char('K'),
//             Key::Char('H'),
//             Key::Char('<'),
//             Key::Char('>'),
//             Key::Char('?'),
//             Key::Empty,
//             Key::Empty,
//             Key::Empty,
//             Key::Empty,
//         ]),
//         Layer([
//             Key::Char('`'),
//             Key::Char('$'),
//             Key::Char('&'),
//             Key::Char('*'),
//             Key::Char('^'),
//             Key::Char('@'),
//             Key::Char('%'),
//             Key::Char('+'),
//             Key::Char('|'),
//             Key::Char('£'),
//             Key::Char('\\'),
//             Key::Char('\''),
//             Key::Char('('),
//             Key::Char(')'),
//             Key::Char('='),
//             Key::Char('#'),
//             Key::Char('{'),
//             Key::Char('}'),
//             Key::Char('-'),
//             Key::Char('!'),
//             Key::Empty,
//             Key::Char('~'),
//             Key::Char('_'),
//             Key::Char('"'),
//             Key::Empty,
//             Key::Empty,
//             Key::Char('['),
//             Key::Char(']'),
//             Key::Empty,
//             Key::Char('¬'),
//             Key::Empty,
//             Key::Empty,
//             Key::Empty,
//             Key::Empty,
//         ]),
//         Layer([
//             Key::Empty,
//             Key::Char('7'),
//             Key::Char('8'),
//             Key::Char('9'),
//             Key::Empty,
//             Key::Empty,
//             Key::Empty,
//             Key::Empty,
//             Key::Empty,
//             Key::Empty,
//             Key::Char('0'),
//             Key::Char('1'),
//             Key::Char('2'),
//             Key::Char('3'),
//             Key::Empty,
//             Key::Empty,
//             Key::Empty,
//             Key::Empty,
//             Key::Empty,
//             Key::Empty,
//             Key::Empty,
//             Key::Char('4'),
//             Key::Char('5'),
//             Key::Char('6'),
//             Key::Empty,
//             Key::Empty,
//             Key::Empty,
//             Key::Empty,
//             Key::Empty,
//             Key::Empty,
//             Key::Empty,
//             Key::Empty,
//             Key::Empty,
//             Key::Empty,
//         ]),
//     ],
// };

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    Char(char),
    Empty,
    Layer(usize),
    // Repeat,
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Char(c) if c.is_ascii_lowercase() || c.is_ascii_digit() => {
                write!(f, "KC_{}", c.to_ascii_uppercase())
            }
            Self::Char(c) if c.is_ascii_uppercase() => write!(f, "LSFT(KC_{})", c),
            Self::Char(',') => write!(f, "KC_COMM"),
            Self::Char('.') => write!(f, "KC_DOT"),
            Self::Char('\'') => write!(f, "KC_QUOT"),
            Self::Char('"') => write!(f, "LSFT(KC_2)"),
            Self::Char('?') => write!(f, "LSFT(KC_SLSH)"),
            Self::Char('!') => write!(f, "LSFT(KC_1)"),
            Self::Char(';') => write!(f, "KC_SCLN"),
            Self::Char(':') => write!(f, "LSFT(KC_SCLN)"),
            Self::Char('\\') => write!(f, "KC_NUBS"),
            Self::Char('/') => write!(f, "KC_SLSH"),
            Self::Char('<') => write!(f, "LSFT(KC_COMM)"),
            Self::Char('>') => write!(f, "LSFT(KC_DOT)"),
            Self::Char('[') => write!(f, "KC_LBRC"),
            Self::Char(']') => write!(f, "KC_RBRC"),
            Self::Char('{') => write!(f, "LSFT(KC_LBRC)"),
            Self::Char('}') => write!(f, "LSFT(KC_RBRC)"),
            Self::Char('(') => write!(f, "LSFT(KC_9)"),
            Self::Char(')') => write!(f, "LSFT(KC_0)"),
            Self::Char('|') => write!(f, "LSFT(KC_NUBS)"),
            Self::Char('#') => write!(f, "KC_NUHS"),
            Self::Char('@') => write!(f, "LSFT(KC_QUOT)"),
            Self::Char('~') => write!(f, "LSFT(KC_NUHS)"),
            Self::Char('$') => write!(f, "LSFT(KC_4)"),
            Self::Char('&') => write!(f, "LSFT(KC_7)"),
            Self::Char('*') => write!(f, "LSFT(KC_8)"),
            Self::Char('^') => write!(f, "LSFT(KC_6)"),
            Self::Char('+') => write!(f, "LSFT(KC_EQL)"),
            Self::Char('%') => write!(f, "LSFT(KC_5)"),
            Self::Char('£') => write!(f, "LSFT(KC_3)"),
            Self::Char('`') => write!(f, "KC_GRV"),
            Self::Char('-') => write!(f, "KC_MINS"),
            Self::Char('_') => write!(f, "LSFT(KC_MINS)"),
            Self::Char('=') => write!(f, "KC_EQL"),
            Self::Char('¬') => write!(f, "LSFT(KC_GRV)"),
            Self::Char(' ') => write!(f, "KC_SPC"),
            Self::Empty => write!(f, "KC_NO"),
            Self::Layer(i) => write!(f, "OSL({})", i),
            _ => Err(fmt::Error),
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
            "KC_COMM" => Ok(Self::Char(',')),
            "KC_DOT" => Ok(Self::Char('.')),
            "KC_QUOT" => Ok(Self::Char('\'')),
            "LSFT(KC_2)" => Ok(Self::Char('"')),
            "LSFT(KC_SLSH)" => Ok(Self::Char('?')),
            "LSFT(KC_1)" => Ok(Self::Char('!')),
            "KC_SCLN" => Ok(Self::Char(';')),
            "LSFT(KC_SCLN)" => Ok(Self::Char(':')),
            "KC_NUBS" => Ok(Self::Char('\\')),
            "KC_SLSH" => Ok(Self::Char('/')),
            "LSFT(KC_COMM)" => Ok(Self::Char('<')),
            "LSFT(KC_DOT)" => Ok(Self::Char('>')),
            "KC_LBRC" => Ok(Self::Char('[')),
            "KC_RBRC" => Ok(Self::Char(']')),
            "LSFT(KC_LBRC)" => Ok(Self::Char('{')),
            "LSFT(KC_RBRC)" => Ok(Self::Char('}')),
            "LSFT(KC_9)" => Ok(Self::Char('(')),
            "LSFT(KC_0)" => Ok(Self::Char(')')),
            "LSFT(KC_NUBS)" => Ok(Self::Char('|')),
            "KC_NUHS" => Ok(Self::Char('#')),
            "LSFT(KC_QUOT)" => Ok(Self::Char('@')),
            "LSFT(KC_NUHS)" => Ok(Self::Char('~')),
            "LSFT(KC_4)" => Ok(Self::Char('$')),
            "LSFT(KC_7)" => Ok(Self::Char('&')),
            "LSFT(KC_8)" => Ok(Self::Char('*')),
            "LSFT(KC_6)" => Ok(Self::Char('^')),
            "LSFT(KC_EQL)" => Ok(Self::Char('+')),
            "LSFT(KC_5)" => Ok(Self::Char('%')),
            "LSFT(KC_3)" => Ok(Self::Char('£')),
            "KC_GRV" => Ok(Self::Char('`')),
            "KC_MINS" => Ok(Self::Char('-')),
            "LSFT(KC_MINS)" => Ok(Self::Char('_')),
            "KC_EQL" => Ok(Self::Char('=')),
            "LSFT(KC_GRV)" => Ok(Self::Char('¬')),
            "KC_SPC" => Ok(Self::Char(' ')),
            "KC_NO" => Ok(Self::Empty),
            s => {
                let cap = KC_RE
                    .captures(s)
                    .ok_or_else(|| ParseError::UnknownValue(s.to_string()))?;
                if let Some(c) = cap.name("upper") {
                    let c = c.as_str().chars().next().unwrap();
                    Ok(Self::Char(c))
                } else if let Some(c) = cap.name("lower") {
                    let c = c.as_str().chars().next().unwrap();
                    Ok(Self::Char(c.to_ascii_lowercase()))
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

pub type KeyCost = Layer<u8>;
pub type NextKeyCost = Layer<Layer<u8>>;
