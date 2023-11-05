use arrayvec::ArrayVec;
use encoding_rs::WINDOWS_1252;
use enum_map::Enum;
use enum_map::EnumMap;
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};

use std::collections::BTreeSet;
use std::collections::VecDeque;
use std::fmt;
use std::iter::FusedIterator;
use std::mem::{discriminant, Discriminant};
use std::ops::{Index, IndexMut};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::{fs, io};

pub const NUM_KEYS: u8 = 34;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Enum)]
#[repr(transparent)]
pub struct Win1252Char(u8);

impl TryFrom<&'_ str> for Win1252Char {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (out, _, has_errors) = WINDOWS_1252.encode(value);
        if has_errors || out.len() != 1 {
            Err(())
        } else {
            Ok(Win1252Char(out[0]))
        }
    }
}

impl TryFrom<char> for Win1252Char {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let buf = value.to_string();
        let (out, _, has_errors) = WINDOWS_1252.encode(&buf);
        if has_errors || out.len() != 1 {
            Err(())
        } else {
            Ok(Win1252Char(out[0]))
        }
    }
}

impl From<Win1252Char> for char {
    fn from(win_c: Win1252Char) -> Self {
        let buf = [win_c.0];
        let (out, _, has_errors) = WINDOWS_1252.decode(&buf);
        assert!(!has_errors);
        assert!(out.chars().count() == 1);
        out.chars().next().unwrap()
    }
}

impl ToString for Win1252Char {
    fn to_string(&self) -> String {
        let buf = [self.0];
        let (out, _, has_errors) = WINDOWS_1252.decode(&buf);
        assert!(!has_errors);
        out.into()
    }
}

lazy_static! {
    static ref KC_RE: Regex =
        Regex::new(r"^LSFT\((?P<shifted>[^()]+)\)|OSL\((?P<layer>\d+)\)$",).unwrap();
}

pub const HOMING: [KeyCode; 7] = [
    KeyCode::F,
    KeyCode::J,
    KeyCode::T,
    KeyCode::N,
    KeyCode::U,
    KeyCode::H,
    KeyCode::Space,
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
    Enter,
    Tab,
}

const A: Win1252Char = Win1252Char(0x61);
const B: Win1252Char = Win1252Char(0x62);
const C: Win1252Char = Win1252Char(0x63);
const D: Win1252Char = Win1252Char(0x64);
const E: Win1252Char = Win1252Char(0x65);
const F: Win1252Char = Win1252Char(0x66);
const G: Win1252Char = Win1252Char(0x67);
const H: Win1252Char = Win1252Char(0x68);
const I: Win1252Char = Win1252Char(0x69);
const J: Win1252Char = Win1252Char(0x6A);
const K: Win1252Char = Win1252Char(0x6B);
const L: Win1252Char = Win1252Char(0x6C);
const M: Win1252Char = Win1252Char(0x6D);
const N: Win1252Char = Win1252Char(0x6E);
const O: Win1252Char = Win1252Char(0x6F);
const P: Win1252Char = Win1252Char(0x70);
const Q: Win1252Char = Win1252Char(0x71);
const R: Win1252Char = Win1252Char(0x72);
const S: Win1252Char = Win1252Char(0x73);
const T: Win1252Char = Win1252Char(0x74);
const U: Win1252Char = Win1252Char(0x75);
const V: Win1252Char = Win1252Char(0x76);
const W: Win1252Char = Win1252Char(0x77);
const X: Win1252Char = Win1252Char(0x78);
const Y: Win1252Char = Win1252Char(0x79);
const Z: Win1252Char = Win1252Char(0x7A);
const DIGIT0: Win1252Char = Win1252Char(0x30);
const DIGIT1: Win1252Char = Win1252Char(0x31);
const DIGIT2: Win1252Char = Win1252Char(0x32);
const DIGIT3: Win1252Char = Win1252Char(0x33);
const DIGIT4: Win1252Char = Win1252Char(0x34);
const DIGIT5: Win1252Char = Win1252Char(0x35);
const DIGIT6: Win1252Char = Win1252Char(0x36);
const DIGIT7: Win1252Char = Win1252Char(0x37);
const DIGIT8: Win1252Char = Win1252Char(0x38);
const DIGIT9: Win1252Char = Win1252Char(0x39);
const COMMA: Win1252Char = Win1252Char(0x2C);
const DOT: Win1252Char = Win1252Char(0x2E);
const APOSTROPHE: Win1252Char = Win1252Char(0x27);
const SEMICOLON: Win1252Char = Win1252Char(0x3B);
const BACKSLASH: Win1252Char = Win1252Char(0x5C);
const SLASH: Win1252Char = Win1252Char(0x2F);
const LEFTSQUAREBRACKET: Win1252Char = Win1252Char(0x5B);
const RIGHTSQUAREBRACKET: Win1252Char = Win1252Char(0x5D);
const HASH: Win1252Char = Win1252Char(0x23);
const GRAVE: Win1252Char = Win1252Char(0x60);
const MINUS: Win1252Char = Win1252Char(0x2D);
const EQUALS: Win1252Char = Win1252Char(0x3D);
pub const SPACE: Win1252Char = Win1252Char(0x20);
pub const ENTER: Win1252Char = Win1252Char(0x0A);
pub const TAB: Win1252Char = Win1252Char(0x09);
const SHIFT_A: Win1252Char = Win1252Char(0x41);
const SHIFT_B: Win1252Char = Win1252Char(0x42);
const SHIFT_C: Win1252Char = Win1252Char(0x43);
const SHIFT_D: Win1252Char = Win1252Char(0x44);
const SHIFT_E: Win1252Char = Win1252Char(0x45);
const SHIFT_F: Win1252Char = Win1252Char(0x46);
const SHIFT_G: Win1252Char = Win1252Char(0x47);
const SHIFT_H: Win1252Char = Win1252Char(0x48);
const SHIFT_I: Win1252Char = Win1252Char(0x49);
const SHIFT_J: Win1252Char = Win1252Char(0x4A);
const SHIFT_K: Win1252Char = Win1252Char(0x4B);
const SHIFT_L: Win1252Char = Win1252Char(0x4C);
const SHIFT_M: Win1252Char = Win1252Char(0x4D);
const SHIFT_N: Win1252Char = Win1252Char(0x4E);
const SHIFT_O: Win1252Char = Win1252Char(0x4F);
const SHIFT_P: Win1252Char = Win1252Char(0x50);
const SHIFT_Q: Win1252Char = Win1252Char(0x51);
const SHIFT_R: Win1252Char = Win1252Char(0x52);
const SHIFT_S: Win1252Char = Win1252Char(0x53);
const SHIFT_T: Win1252Char = Win1252Char(0x54);
const SHIFT_U: Win1252Char = Win1252Char(0x55);
const SHIFT_V: Win1252Char = Win1252Char(0x56);
const SHIFT_W: Win1252Char = Win1252Char(0x57);
const SHIFT_X: Win1252Char = Win1252Char(0x58);
const SHIFT_Y: Win1252Char = Win1252Char(0x59);
const SHIFT_Z: Win1252Char = Win1252Char(0x5A);
const SHIFT_DIGIT0: Win1252Char = Win1252Char(0x29);
const SHIFT_DIGIT1: Win1252Char = Win1252Char(0x21);
const SHIFT_DIGIT2: Win1252Char = Win1252Char(0x22);
const SHIFT_DIGIT3: Win1252Char = Win1252Char(0xA3);
const SHIFT_DIGIT4: Win1252Char = Win1252Char(0x24);
const SHIFT_DIGIT5: Win1252Char = Win1252Char(0x25);
const SHIFT_DIGIT6: Win1252Char = Win1252Char(0x5E);
const SHIFT_DIGIT7: Win1252Char = Win1252Char(0x26);
const SHIFT_DIGIT8: Win1252Char = Win1252Char(0x2A);
const SHIFT_DIGIT9: Win1252Char = Win1252Char(0x28);
const SHIFT_COMMA: Win1252Char = Win1252Char(0x3C);
const SHIFT_DOT: Win1252Char = Win1252Char(0x3E);
const SHIFT_APOSTROPHE: Win1252Char = Win1252Char(0x40);
const SHIFT_SEMICOLON: Win1252Char = Win1252Char(0x3A);
const SHIFT_BACKSLASH: Win1252Char = Win1252Char(0x7C);
const SHIFT_SLASH: Win1252Char = Win1252Char(0x3F);
const SHIFT_LEFTSQUAREBRACKET: Win1252Char = Win1252Char(0x7B);
const SHIFT_RIGHTSQUAREBRACKET: Win1252Char = Win1252Char(0x7D);
const SHIFT_HASH: Win1252Char = Win1252Char(0x7E);
const SHIFT_GRAVE: Win1252Char = Win1252Char(0xAC);
const SHIFT_MINUS: Win1252Char = Win1252Char(0x5F);
const SHIFT_EQUALS: Win1252Char = Win1252Char(0x2B);
const SHIFT_SPACE: Win1252Char = Win1252Char(0x20);
const SHIFT_ENTER: Win1252Char = Win1252Char(0x0A);
const SHIFT_TAB: Win1252Char = Win1252Char(0x09);

impl KeyCode {
    fn typed_char(self) -> Win1252Char {
        match self {
            Self::A => A,
            Self::B => B,
            Self::C => C,
            Self::D => D,
            Self::E => E,
            Self::F => F,
            Self::G => G,
            Self::H => H,
            Self::I => I,
            Self::J => J,
            Self::K => K,
            Self::L => L,
            Self::M => M,
            Self::N => N,
            Self::O => O,
            Self::P => P,
            Self::Q => Q,
            Self::R => R,
            Self::S => S,
            Self::T => T,
            Self::U => U,
            Self::V => V,
            Self::W => W,
            Self::X => X,
            Self::Y => Y,
            Self::Z => Z,
            Self::Digit0 => DIGIT0,
            Self::Digit1 => DIGIT1,
            Self::Digit2 => DIGIT2,
            Self::Digit3 => DIGIT3,
            Self::Digit4 => DIGIT4,
            Self::Digit5 => DIGIT5,
            Self::Digit6 => DIGIT6,
            Self::Digit7 => DIGIT7,
            Self::Digit8 => DIGIT8,
            Self::Digit9 => DIGIT9,
            Self::Comma => COMMA,
            Self::Dot => DOT,
            Self::Apostrophe => APOSTROPHE,
            Self::Semicolon => SEMICOLON,
            Self::Backslash => BACKSLASH,
            Self::Slash => SLASH,
            Self::LeftSquareBracket => LEFTSQUAREBRACKET,
            Self::RightSquareBracket => RIGHTSQUAREBRACKET,
            Self::Hash => HASH,
            Self::Grave => GRAVE,
            Self::Minus => MINUS,
            Self::Equals => EQUALS,
            Self::Space => SPACE,
            Self::Enter => ENTER,
            Self::Tab => TAB,
        }
    }

    fn shifted_char(self) -> Win1252Char {
        match self {
            Self::A => SHIFT_A,
            Self::B => SHIFT_B,
            Self::C => SHIFT_C,
            Self::D => SHIFT_D,
            Self::E => SHIFT_E,
            Self::F => SHIFT_F,
            Self::G => SHIFT_G,
            Self::H => SHIFT_H,
            Self::I => SHIFT_I,
            Self::J => SHIFT_J,
            Self::K => SHIFT_K,
            Self::L => SHIFT_L,
            Self::M => SHIFT_M,
            Self::N => SHIFT_N,
            Self::O => SHIFT_O,
            Self::P => SHIFT_P,
            Self::Q => SHIFT_Q,
            Self::R => SHIFT_R,
            Self::S => SHIFT_S,
            Self::T => SHIFT_T,
            Self::U => SHIFT_U,
            Self::V => SHIFT_V,
            Self::W => SHIFT_W,
            Self::X => SHIFT_X,
            Self::Y => SHIFT_Y,
            Self::Z => SHIFT_Z,
            Self::Digit0 => SHIFT_DIGIT0,
            Self::Digit1 => SHIFT_DIGIT1,
            Self::Digit2 => SHIFT_DIGIT2,
            Self::Digit3 => SHIFT_DIGIT3,
            Self::Digit4 => SHIFT_DIGIT4,
            Self::Digit5 => SHIFT_DIGIT5,
            Self::Digit6 => SHIFT_DIGIT6,
            Self::Digit7 => SHIFT_DIGIT7,
            Self::Digit8 => SHIFT_DIGIT8,
            Self::Digit9 => SHIFT_DIGIT9,
            Self::Comma => SHIFT_COMMA,
            Self::Dot => SHIFT_DOT,
            Self::Apostrophe => SHIFT_APOSTROPHE,
            Self::Semicolon => SHIFT_SEMICOLON,
            Self::Backslash => SHIFT_BACKSLASH,
            Self::Slash => SHIFT_SLASH,
            Self::LeftSquareBracket => SHIFT_LEFTSQUAREBRACKET,
            Self::RightSquareBracket => SHIFT_RIGHTSQUAREBRACKET,
            Self::Hash => SHIFT_HASH,
            Self::Grave => SHIFT_GRAVE,
            Self::Minus => SHIFT_MINUS,
            Self::Equals => SHIFT_EQUALS,
            Self::Space => SHIFT_SPACE,
            Self::Enter => SHIFT_ENTER,
            Self::Tab => SHIFT_TAB,
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
            Self::Enter => write!(f, "KC_ENT"),
            Self::Tab => write!(f, "KC_TAB"),
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
            "KC_ENT" => Ok(Self::Enter),
            "KC_TAB" => Ok(Self::Tab),
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
    Layer(u8),
    // Repeat,
}

impl Key {
    pub fn typed_char(self, shifted: bool) -> Option<Win1252Char> {
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

#[derive(Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct Layer<T>(pub(crate) [T; NUM_KEYS as usize]);

impl<T> Layer<T> {
    pub(crate) fn iter(&self) -> impl Iterator<Item = &T> {
        self.0.iter()
    }
}

impl<T: fmt::Debug> Layer<T> {
    pub(crate) fn from_fun<F: FnMut(u8) -> T>(f: F) -> Self {
        Self((0..NUM_KEYS).map(f).collect::<Vec<_>>().try_into().unwrap())
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
                        expected: NUM_KEYS as usize,
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

impl<T> Index<u8> for Layer<T> {
    type Output = T;

    fn index(&self, index: u8) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl<T> IndexMut<u8> for Layer<T> {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}

impl<T: fmt::Debug> fmt::Debug for Layer<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            writeln!(f, "Layer([")?;
            for r in 0..3 {
                write!(f, "    ")?;
                for c in 0..5 {
                    write!(f, "{:4.2?}, ", self.0[r * 10 + c])?;
                }
                write!(f, " ")?;
                for c in 5..10 {
                    write!(f, "{:4.2?}, ", self.0[r * 10 + c])?;
                }
                writeln!(f)?;
            }
            write!(f, "    ")?;
            write!(f, "{}", " ".repeat(6 * 3))?;
            let r = 3;
            for c in 0..2 {
                write!(f, "{:4.2?}, ", self.0[r * 10 + c])?;
            }
            write!(f, " ")?;
            for c in 2..4 {
                write!(f, "{:4.2?}, ", self.0[r * 10 + c])?;
            }
            writeln!(f)?;
            write!(f, "])")
        } else {
            write!(f, "Layer(")?;
            let mut list = f.debug_list();
            list.entries(self.0.iter());
            list.finish()?;
            write!(f, ")")
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "serde_json::Value")]
#[serde(into = "serde_json::Value")]
#[repr(transparent)]
pub struct Layout {
    pub(crate) layers: Vec<Layer<Key>>,
}

impl Layout {
    pub(crate) fn iter(&self) -> impl Iterator<Item = &Layer<Key>> {
        self.layers.iter()
    }

    fn has_key(&self, key: Key) -> bool {
        self.iter().any(|l| l.iter().any(|&k| k == key))
    }

    pub fn hamming_dist(&self, other: &Self) -> u8 {
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
                            .collect::<Result<Vec<_>, _>>(),
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

impl Index<u8> for Layout {
    type Output = Layer<Key>;

    fn index(&self, index: u8) -> &Self::Output {
        &self.layers[index as usize]
    }
}

impl IndexMut<u8> for Layout {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        &mut self.layers[index as usize]
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Enum)]
#[repr(u8)]
pub enum Finger {
    Pinky,
    Ring,
    Middle,
    Index,
    Thumb,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Enum)]
#[repr(u8)]
pub enum Hand {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Enum)]
#[repr(u8)]
pub enum Digit {
    LeftPinky,
    LeftRing,
    LeftMiddle,
    LeftIndex,
    LeftThumb,
    RightPinky,
    RightRing,
    RightMiddle,
    RightIndex,
    RightThumb,
}

impl Digit {
    pub fn hand(self) -> Hand {
        if self as u8 >= 5 {
            Hand::Right
        } else {
            Hand::Left
        }
    }

    pub fn finger(self) -> Finger {
        unsafe { std::mem::transmute(self as u8 % 5) }
    }

    pub fn new(hand: Hand, finger: Finger) -> Self {
        unsafe { std::mem::transmute(hand as u8 * 5 + finger as u8) }
    }
}

pub fn finger_for_pos(row: u8, col: u8) -> Digit {
    if row == 3 {
        match col {
            0 | 1 => Digit::new(Hand::Left, Finger::Thumb),
            2 | 3 => Digit::new(Hand::Right, Finger::Thumb),
            _ => panic!("invalid column {} for row {}", col, row),
        }
    } else {
        match col {
            0 => Digit::new(Hand::Left, Finger::Pinky),
            1 => Digit::new(Hand::Left, Finger::Ring),
            2 => Digit::new(Hand::Left, Finger::Middle),
            3 | 4 => Digit::new(Hand::Left, Finger::Index),
            5 | 6 => Digit::new(Hand::Right, Finger::Index),
            7 => Digit::new(Hand::Right, Finger::Middle),
            8 => Digit::new(Hand::Right, Finger::Ring),
            9 => Digit::new(Hand::Right, Finger::Pinky),
            _ => panic!("invalid column {} for row {}", col, row),
        }
    }
}

#[derive(Clone)]
pub struct Keys<'l, I> {
    layout: &'l AnnotatedLayout,
    chars: I,
    cur_layer: u8,
    cur_shifted: bool,
    buf: VecDeque<TypingEvent>,
}

impl<'l, I> Keys<'l, I>
where
    I: Iterator<Item = Win1252Char>,
{
    fn extend_buf_to(&mut self, n: usize) {
        while self.buf.len() <= n {
            if !self.handle_next() {
                break;
            }
        }
    }

    fn handle_next(&mut self) -> bool {
        if let Some(c) = self.chars.next() {
            match self.layout.char_idx[c].last() {
                Some(&CharIdxEntry {
                    layer,
                    pos,
                    shifted,
                }) => {
                    // A typable character.
                    if self.cur_layer != 0 && layer != self.cur_layer {
                        self.buf.push_back(TypingEvent::Release(
                            self.layout.layer_idx[self.cur_layer as usize],
                        ));
                        self.cur_layer = 0;
                    }
                    if self.cur_shifted && !shifted {
                        self.buf
                            .push_back(TypingEvent::Release(self.layout.shift_idx.unwrap()));
                        self.cur_shifted = false;
                    }

                    if shifted && !self.cur_shifted {
                        // The shift key is on the home layer.
                        if self.cur_layer != 0 {
                            self.buf.push_back(TypingEvent::Release(
                                self.layout.layer_idx[self.cur_layer as usize],
                            ));
                            self.cur_layer = 0;
                        }
                        self.buf
                            .push_back(TypingEvent::Hold(self.layout.shift_idx.unwrap()));
                        self.cur_shifted = true;
                    }
                    if layer != 0 && self.cur_layer != layer {
                        self.buf
                            .push_back(TypingEvent::Hold(self.layout.layer_idx[layer as usize]));
                        self.cur_layer = layer;
                    }

                    self.buf.push_back(TypingEvent::Tap(pos));
                    true
                }
                None => {
                    // An untypable character.
                    if self.cur_layer != 0 {
                        self.buf.push_back(TypingEvent::Release(
                            self.layout.layer_idx[self.cur_layer as usize],
                        ));
                        self.cur_layer = 0;
                    }
                    if self.cur_shifted {
                        self.buf
                            .push_back(TypingEvent::Release(self.layout.shift_idx.unwrap()));
                        self.cur_shifted = false;
                    }
                    self.buf.push_back(TypingEvent::Unknown);
                    true
                }
            }
        } else {
            // No more characters.
            if self.cur_layer != 0 {
                self.buf.push_back(TypingEvent::Release(
                    self.layout.layer_idx[self.cur_layer as usize],
                ));
                self.cur_layer = 0;
            }
            if self.cur_shifted {
                self.buf
                    .push_back(TypingEvent::Release(self.layout.shift_idx.unwrap()));
                self.cur_shifted = false;
            }
            false
        }
    }
}

impl<'l, I> Iterator for Keys<'l, I>
where
    I: Iterator<Item = Win1252Char>,
{
    type Item = TypingEvent;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(e) = self.buf.pop_front() {
            return Some(e);
        }

        self.handle_next();
        self.buf.pop_front()
    }

    /// At least one keypress will be yielded for each character in the input,
    /// either a `Tap` of the corresponding key or `Unknown`.
    /// More may be generated to switch layers etc.
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (l, _) = self.chars.size_hint();
        (l + self.buf.len(), None)
    }
}
impl<'l, I> FusedIterator for Keys<'l, I> where I: FusedIterator + Iterator<Item = Win1252Char> {}

impl<'l, I> LookaheadIterator for Keys<'l, I>
where
    I: Iterator<Item = Win1252Char>,
{
    fn peek_nth(&mut self, n: usize) -> Option<&Self::Item> {
        self.extend_buf_to(n);
        self.buf.get(n)
    }

    fn remove_nth(&mut self, n: usize) -> Option<Self::Item> {
        self.extend_buf_to(n);
        self.buf.remove(n)
    }
}

pub trait LookaheadIterator: Iterator {
    /// Peek at the nth element of the iterator.
    ///
    /// This will _not_ consume any elements from the iterator,
    /// but may consume from an underlying one.
    fn peek_nth(&mut self, n: usize) -> Option<&Self::Item>;

    /// Pop the nth element of the iterator.
    ///
    /// This will consume _only_ the nth element, not any earlier ones.
    fn remove_nth(&mut self, n: usize) -> Option<Self::Item>;
}

#[derive(Clone)]
pub struct NPeekable<I: Iterator> {
    inner: I,
    buf: VecDeque<I::Item>,
}

impl<I: Iterator> NPeekable<I> {
    fn extend_buf_to(&mut self, n: usize) {
        if n >= self.buf.len() {
            self.buf
                .extend(self.inner.by_ref().take(n + 1 - self.buf.len()));
        }
    }
}

impl<I: Iterator> Iterator for NPeekable<I> {
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if let elem @ Some(_) = self.buf.pop_front() {
            return elem;
        }

        self.inner.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, upper) = self.inner.size_hint();
        let buf_len = self.buf.len();
        (
            lower.saturating_add(buf_len),
            upper.map(|u| u.saturating_add(buf_len)),
        )
    }

    fn count(self) -> usize {
        self.buf.len() + self.inner.count()
    }

    fn last(mut self) -> Option<Self::Item> {
        self.inner.last().or_else(|| self.buf.pop_back())
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        if n < self.buf.len() {
            self.buf.drain(..=n).next_back()
        } else {
            let out = self.inner.nth(n - self.buf.len());
            self.buf.clear();
            out
        }
    }

    fn fold<B, F>(self, init: B, mut f: F) -> B
    where
        F: FnMut(B, Self::Item) -> B,
    {
        let accum = self.buf.into_iter().fold(init, &mut f);
        self.inner.fold(accum, f)
    }
}

impl<I: Iterator> LookaheadIterator for NPeekable<I> {
    fn peek_nth(&mut self, n: usize) -> Option<&Self::Item> {
        self.extend_buf_to(n);
        self.buf.get(n)
    }

    fn remove_nth(&mut self, n: usize) -> Option<Self::Item> {
        self.extend_buf_to(n);
        self.buf.remove(n)
    }
}

#[derive(Clone)]
pub struct Oneshot<I> {
    events: I,
}

impl<I> Iterator for Oneshot<I>
where
    I: LookaheadIterator + Iterator<Item = TypingEvent>,
{
    type Item = TypingEvent;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.events.next() {
                Some(TypingEvent::Hold(pos)) => {
                    let mut used = false;
                    for i in 0.. {
                        match self.events.peek_nth(i).copied() {
                            None => panic!("{pos} held but not released"),
                            Some(TypingEvent::Tap(_) | TypingEvent::Unknown) => {
                                if used {
                                    // The hold is used for multiple keys, so should stay a hold.
                                    return Some(TypingEvent::Hold(pos));
                                }
                                used = true;
                            }
                            Some(TypingEvent::Release(pos2)) if pos == pos2 => {
                                match used {
                                    false => {
                                        // The hold is not actually doing anything, so remove it.
                                        // This will break if modifiers require other modifiers to be held,
                                        // for eg. if they are not on the home layer.
                                        self.events.remove_nth(i);
                                    }
                                    true => {
                                        // The hold is used for 1 tap, so should be oneshot.
                                        self.events.remove_nth(i);
                                        return Some(TypingEvent::Tap(pos));
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
                e => {
                    return e;
                }
            }
        }
    }

    /// At most one event will be yielded for each event in the input.
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (_, u) = self.events.size_hint();
        (0, u)
    }
}
impl<I> FusedIterator for Oneshot<I> where
    I: FusedIterator + LookaheadIterator + Iterator<Item = TypingEvent>
{
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypingEvent {
    Tap(u8),
    Hold(u8),
    Release(u8),
    Unknown,
}

pub fn keys<I: IntoIterator<Item = Win1252Char>>(
    layout: &AnnotatedLayout,
    chars: I,
) -> Keys<'_, I::IntoIter> {
    Keys {
        layout,
        chars: chars.into_iter(),
        cur_layer: 0,
        cur_shifted: false,
        buf: VecDeque::new(),
    }
}

pub fn oneshot<I>(events: I) -> Oneshot<I::IntoIter>
where
    I: IntoIterator<Item = TypingEvent>,
    I::IntoIter: LookaheadIterator,
{
    Oneshot {
        events: events.into_iter(),
    }
}

pub fn lookahead<I>(iter: I) -> NPeekable<I::IntoIter>
where
    I: IntoIterator,
{
    NPeekable {
        inner: iter.into_iter(),
        buf: VecDeque::new(),
    }
}

#[derive(Clone, Copy)]
enum OneshotState<T> {
    Persistent(T),
    Oneshot(T),
}

impl<T> OneshotState<T> {
    fn current(self) -> T {
        match self {
            Self::Persistent(val) => val,
            Self::Oneshot(val) => val,
        }
    }

    fn is_oneshot(self) -> bool {
        match self {
            Self::Persistent(_) => false,
            Self::Oneshot(_) => true,
        }
    }
}

impl<T: Default> Default for OneshotState<T> {
    fn default() -> Self {
        Self::Persistent(T::default())
    }
}

#[derive(Clone)]
struct Typist<'l, I> {
    layout: &'l AnnotatedLayout,
    events: I,
    layer_state: OneshotState<u8>,
    shift_state: OneshotState<bool>,
}

impl<'l, I> Typist<'l, I> {
    fn finish_oneshot(&mut self) {
        if self.layer_state.is_oneshot() {
            self.layer_state = OneshotState::default();
        }
        if self.shift_state.is_oneshot() {
            self.shift_state = OneshotState::default();
        }
    }
}

impl<'l, I> Iterator for Typist<'l, I>
where
    I: Iterator<Item = TypingEvent>,
{
    type Item = Win1252Char;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.events.next() {
                None => return None,
                Some(TypingEvent::Tap(pos)) => {
                    let layer = self.layer_state.current();
                    let shifted = self.shift_state.current();

                    match self.layout.layout()[layer][pos] {
                        Key::Empty => {
                            self.finish_oneshot();
                        }
                        Key::Layer(layer) => {
                            self.layer_state = OneshotState::Oneshot(layer);
                        }
                        Key::Shift => {
                            self.shift_state = OneshotState::Oneshot(true);
                        }
                        Key::Shifted(kc) => {
                            self.finish_oneshot();
                            return Some(kc.shifted_char());
                        }
                        Key::Typing(kc) => {
                            self.finish_oneshot();
                            return Some(if shifted {
                                kc.shifted_char()
                            } else {
                                kc.typed_char()
                            });
                        }
                    }
                }
                Some(TypingEvent::Hold(pos)) => {
                    let layer = self.layer_state.current();

                    match self.layout.layout()[layer][pos] {
                        Key::Layer(layer) => {
                            self.layer_state = OneshotState::Persistent(layer);
                        }
                        Key::Shift => {
                            self.shift_state = OneshotState::Persistent(true);
                        }
                        _ => {}
                    }
                }
                Some(TypingEvent::Release(pos)) => {
                    // Assume that the held key was on the home layer
                    // and that only one of each type of key is held at once.
                    match self.layout.layout()[0][pos] {
                        Key::Layer(layer) => {
                            self.layer_state = OneshotState::default();
                        }
                        Key::Shift => {
                            self.shift_state = OneshotState::default();
                        }
                        _ => {}
                    }
                }
                Some(TypingEvent::Unknown) => self.finish_oneshot(),
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        // The resulting iterator cannot be longer than the event iterator
        // as each event can type at most one character.
        (0, self.events.size_hint().1)
    }
}
impl<'l, I> FusedIterator for Typist<'l, I> where I: Iterator<Item = TypingEvent> + FusedIterator {}

fn type_events<I>(layout: &AnnotatedLayout, events: I) -> Typist<I::IntoIter>
where
    I: IntoIterator<Item = TypingEvent>,
{
    Typist {
        layout,
        events: events.into_iter(),
        layer_state: OneshotState::default(),
        shift_state: OneshotState::default(),
    }
}

/// Interpret a String as a Vec of bytes encoded using Windows_1252, where each byte represents one char.
/// If any of the chars in the String are not encodable, returns None.
pub fn to_bytes(string: String) -> Option<Vec<Win1252Char>> {
    let (out, _, had_errors) = WINDOWS_1252.encode(&string);

    // SAFETY: Win1252_char is a repr(transparent) wrapper of u8
    // and the bytes came from WINDOWS_1252::encode, so they are valid.
    (!had_errors).then_some(unsafe { std::mem::transmute(out.to_vec()) })
}

// Assumes there is only one intended way of typing each character,
// and that all typable characters have a single-byte representation.
pub type CharIdx = EnumMap<Win1252Char, BTreeSet<CharIdxEntry>>;

/// A way of typing a character. The ordering defines preference -
/// a larger entry is chosen over a smaller one. The layer and pos are also incorporated
/// so that entries are only equal if they are identical.
#[derive(Debug, Clone, Copy, Eq)]
pub struct CharIdxEntry {
    pub layer: u8,
    pub pos: u8,
    pub shifted: bool,
}

impl Ord for CharIdxEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let layer_order = (self.layer == 0).cmp(&(other.layer == 0));
        if !matches!(layer_order, std::cmp::Ordering::Equal) {
            return layer_order;
        }
        let shifted_order = (!self.shifted).cmp(&(!other.shifted));
        if !matches!(shifted_order, std::cmp::Ordering::Equal) {
            return shifted_order;
        }
        // Prefer smaller layer numbers and positions, all else being equal
        let layer_order = other.layer.cmp(&self.layer);
        if !matches!(layer_order, std::cmp::Ordering::Equal) {
            return layer_order;
        }
        let pos_order = other.pos.cmp(&self.pos);
        pos_order
    }
}

impl PartialOrd for CharIdxEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for CharIdxEntry {
    fn eq(&self, other: &Self) -> bool {
        matches!(self.cmp(other), std::cmp::Ordering::Equal)
    }
}

lazy_static! {
    pub static ref NUMBERS: [Win1252Char; 10] =
        ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"].map(|s| s.try_into().unwrap());
    pub static ref LOWER_ALPHA: [Win1252Char; 26] = [
        "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r",
        "s", "t", "u", "v", "w", "x", "y", "z",
    ]
    .map(|s| s.try_into().unwrap());
    pub static ref UPPER_ALPHA: [Win1252Char; 26] = [
        "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R",
        "S", "T", "U", "V", "W", "X", "Y", "Z",
    ]
    .map(|s| s.try_into().unwrap());
}

pub static NUM_LAYOUTS: [[u8; 10]; 22] = [
    [10, 11, 12, 13, 21, 22, 23, 1, 2, 3],
    [14, 11, 12, 13, 21, 22, 23, 1, 2, 3],
    [19, 16, 17, 18, 26, 27, 28, 6, 7, 8],
    [15, 16, 17, 18, 26, 27, 28, 6, 7, 8],
    [20, 21, 22, 23, 11, 12, 13, 1, 2, 3],
    [10, 21, 22, 23, 11, 12, 13, 1, 2, 3],
    [14, 21, 22, 23, 11, 12, 13, 1, 2, 3],
    [29, 26, 27, 28, 16, 17, 18, 6, 7, 8],
    [19, 26, 27, 28, 16, 17, 18, 6, 7, 8],
    [15, 26, 27, 28, 16, 17, 18, 6, 7, 8],
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
    [9, 0, 1, 2, 3, 4, 5, 6, 7, 8],
    [10, 11, 12, 13, 14, 15, 16, 17, 18, 19],
    [19, 10, 11, 12, 13, 14, 15, 16, 17, 18],
    [20, 21, 22, 23, 24, 25, 26, 27, 28, 29],
    [29, 20, 21, 22, 23, 24, 25, 26, 27, 28],
    [10, 11, 12, 13, 23, 26, 16, 17, 18, 19],
    [10, 11, 12, 13, 3, 6, 16, 17, 18, 19],
    [0, 1, 2, 3, 4, 10, 11, 12, 13, 14],
    [10, 11, 12, 13, 14, 20, 21, 22, 23, 24],
    [5, 6, 7, 8, 9, 15, 16, 17, 18, 19],
    [15, 16, 17, 18, 19, 25, 26, 27, 28, 29],
];

#[derive(Debug, Clone)]
pub struct AnnotatedLayout {
    layout: Layout,
    char_idx: CharIdx,
    layer_idx: Vec<u8>,
    shift_idx: Option<u8>,
    num_layout: u8,
    num_layer: u8,
}

impl AnnotatedLayout {
    pub fn layout(&self) -> &Layout {
        &self.layout
    }

    pub fn char_idx(&self) -> &CharIdx {
        &self.char_idx
    }

    pub fn layer_idx(&self) -> &[u8] {
        &self.layer_idx
    }

    pub fn shift_idx(&self) -> &Option<u8> {
        &self.shift_idx
    }

    pub fn num_layout(&self) -> u8 {
        self.num_layout
    }

    pub fn num_layers(&self) -> u8 {
        self.layout.layers.len() as u8
    }

    pub fn swap(&mut self, (layer_a, pos_a): (u8, u8), (layer_b, pos_b): (u8, u8)) {
        if (layer_a, pos_a) == (layer_b, pos_b) {
            return;
        }

        let a = self.layout[layer_a][pos_a];
        let b = self.layout[layer_b][pos_b];

        // Update char_idx.
        // Entries must be removed together and inserted together so that keys which
        // can type the same char are updated correctly.
        let mut changed_entries: ArrayVec<_, 4> = ArrayVec::new();
        if let Some(c) = a.typed_char(false) {
            let mut entry = self.char_idx[c]
                .take(&CharIdxEntry {
                    layer: layer_a,
                    pos: pos_a,
                    shifted: false,
                })
                .expect("typable key should have entry");
            assert!(!entry.shifted);
            assert_eq!(entry.layer, layer_a);
            assert_eq!(entry.pos, pos_a);
            entry.layer = layer_b;
            entry.pos = pos_b;
            changed_entries.push((c, entry));
        }
        if let Some(c) = b.typed_char(false) {
            let mut entry = self.char_idx[c]
                .take(&CharIdxEntry {
                    layer: layer_b,
                    pos: pos_b,
                    shifted: false,
                })
                .expect("typable key should have entry");
            assert!(!entry.shifted);
            assert_eq!(entry.layer, layer_b);
            assert_eq!(entry.pos, pos_b);
            entry.layer = layer_a;
            entry.pos = pos_a;
            changed_entries.push((c, entry));
        }
        if let Some(c) = a.typed_char(true) {
            let mut entry = self.char_idx[c]
                .take(&CharIdxEntry {
                    layer: layer_a,
                    pos: pos_a,
                    shifted: true,
                })
                .expect("typable key should have shifted entry");
            assert!(entry.shifted);
            assert_eq!(entry.layer, layer_a);
            assert_eq!(entry.pos, pos_a);
            entry.layer = layer_b;
            entry.pos = pos_b;
            changed_entries.push((c, entry));
        }
        if let Some(c) = b.typed_char(true) {
            let mut entry = self.char_idx[c]
                .take(&CharIdxEntry {
                    layer: layer_b,
                    pos: pos_b,
                    shifted: true,
                })
                .expect("typable key should have shifted entry");
            assert!(entry.shifted);
            assert_eq!(entry.layer, layer_b);
            assert_eq!(entry.pos, pos_b);
            entry.layer = layer_a;
            entry.pos = pos_a;
            changed_entries.push((c, entry));
        }
        for (c, entry) in changed_entries {
            assert!(self.char_idx[c].insert(entry));
        }

        if let Key::Layer(layer) = a {
            assert_eq!(layer_a, 0);
            assert_eq!(self.layer_idx[layer as usize], pos_a);
            self.layer_idx[layer as usize] = pos_b;
        }
        if let Key::Layer(layer) = b {
            assert_eq!(layer_b, 0);
            assert_eq!(self.layer_idx[layer as usize], pos_b);
            self.layer_idx[layer as usize] = pos_a;
        }
        if let Key::Shift = a {
            assert_eq!(layer_a, 0);
            assert_eq!(self.shift_idx, Some(pos_a));
            self.shift_idx = Some(pos_b);
        }
        if let Key::Shift = b {
            assert_eq!(layer_b, 0);
            assert_eq!(self.shift_idx, Some(pos_b));
            self.shift_idx = Some(pos_a);
        }

        if layer_a == layer_b {
            self.layout[layer_a].0.swap(pos_a as usize, pos_b as usize);
        } else {
            let (layer_low, layer_high, pos_low, pos_high);
            if layer_a > layer_b {
                (layer_low, pos_low, layer_high, pos_high) = (layer_b, pos_b, layer_a, pos_a);
            } else {
                (layer_low, pos_low, layer_high, pos_high) = (layer_a, pos_a, layer_b, pos_b);
            }
            assert!(layer_low < layer_high);
            // Split the layers so we can safely have mutable references
            // to two parts of it.
            let (left, right) = self.layout.layers.split_at_mut(layer_low as usize + 1);
            assert_eq!(left.len(), layer_low as usize + 1);
            std::mem::swap(
                &mut left.last_mut().unwrap()[pos_low],
                &mut right[(layer_high - layer_low) as usize - 1][pos_high],
            );
        }

        assert_eq!(b, self.layout[layer_a][pos_a]);
        assert_eq!(a, self.layout[layer_b][pos_b]);
    }

    pub fn switch_to_num_layout(&mut self, new_layout: u8) {
        debug_assert_eq!(
            self.num_layout,
            NUM_LAYOUTS
                .iter()
                .position(|&l| {
                    NUMBERS
                        .iter()
                        .map(|&c| {
                            let entry = self.char_idx[c].last().unwrap();
                            assert_eq!(entry.layer, self.num_layer);
                            entry.pos
                        })
                        .zip(l)
                        .all(|(actual, desired)| actual == desired)
                })
                .unwrap() as u8
        );

        for (i, &new_pos) in NUM_LAYOUTS[new_layout as usize].iter().enumerate() {
            let old_pos = self.char_idx[NUMBERS[i]].last().unwrap().pos;
            assert_eq!(
                self.char_idx[NUMBERS[i]].last(),
                Some(&CharIdxEntry {
                    layer: self.num_layer,
                    pos: old_pos,
                    shifted: false
                })
            );
            assert_eq!(
                self.layout[self.num_layer][old_pos].typed_char(false),
                Some(NUMBERS[i])
            );
            self.swap((self.num_layer, old_pos), (self.num_layer, new_pos));
            assert_eq!(
                self.layout[self.num_layer][new_pos].typed_char(false),
                Some(NUMBERS[i])
            );
            assert_eq!(
                self.char_idx[NUMBERS[i]].last(),
                Some(&CharIdxEntry {
                    layer: self.num_layer,
                    pos: new_pos,
                    shifted: false
                })
            );
        }
        self.num_layout = new_layout;

        debug_assert_eq!(
            self.num_layout,
            NUM_LAYOUTS
                .iter()
                .position(|&l| {
                    NUMBERS
                        .iter()
                        .map(|&c| {
                            let entry = self.char_idx[c].last().unwrap();
                            assert_eq!(entry.layer, self.num_layer);
                            entry.pos
                        })
                        .zip(l)
                        .all(|(actual, desired)| actual == desired)
                })
                .unwrap() as u8
        );
    }
}

impl From<Layout> for AnnotatedLayout {
    fn from(layout: Layout) -> Self {
        let mut char_idx = CharIdx::default();
        for (i, l) in (0..).zip(layout.iter()) {
            for (j, k) in (0..).zip(l.iter()) {
                if let Some(c) = k.typed_char(false) {
                    char_idx[c].insert(CharIdxEntry {
                        layer: i,
                        pos: j,
                        shifted: false,
                    });
                }
                if let Some(c) = k.typed_char(true) {
                    char_idx[c].insert(CharIdxEntry {
                        layer: i,
                        pos: j,
                        shifted: true,
                    });
                }
            }
        }
        let layer_idx = (0..)
            .zip(layout[0].iter())
            .filter_map(move |(j, k)| match k {
                Key::Layer(n) => Some((*n, j)),
                _ => None,
            })
            .fold(vec![0; layout.layers.len()], |mut a, (n, j)| {
                a[n as usize] = j;
                a
            });
        let shift_idx = (0..)
            .zip(layout[0].iter())
            .find_map(|(i, k)| matches!(k, Key::Shift).then_some(i));

        let (num_layer, num_layout) = match char_idx[NUMBERS[0]].last().map(|e| e.layer) {
            Some(num_layer) => {
                let num_layout = NUM_LAYOUTS
                    .iter()
                    .position(|&l| {
                        NUMBERS
                            .iter()
                            .map(|&c| {
                                let entry = char_idx[c].last().unwrap();
                                assert_eq!(entry.layer, num_layer);
                                entry.pos
                            })
                            .zip(l)
                            .all(|(actual, desired)| actual == desired)
                    })
                    .unwrap() as u8;
                (num_layer, num_layout)
            }
            None => (0, 0),
        };

        Self {
            layout,
            char_idx,
            layer_idx,
            shift_idx,
            num_layout,
            num_layer,
        }
    }
}

impl From<AnnotatedLayout> for Layout {
    fn from(layout: AnnotatedLayout) -> Self {
        layout.layout
    }
}

fn read_corpus_impl<P: AsRef<Path>>(corpus: &mut Vec<Vec<Win1252Char>>, path: P) -> io::Result<()> {
    let path = path.as_ref();
    if path.is_dir() {
        for entry in path.read_dir()? {
            read_corpus_impl(corpus, entry?.path())?;
        }
    } else {
        let string = fs::read_to_string(path)?;
        corpus.push(
            to_bytes(string).unwrap_or_else(|| panic!("unable to encode {}", path.display())),
        );
    }

    Ok(())
}

fn read_named_corpus_impl<P: AsRef<Path>>(
    corpus: &mut Vec<(PathBuf, Vec<Win1252Char>)>,
    path: P,
) -> io::Result<()> {
    let path = path.as_ref();
    if path.is_dir() {
        for entry in path.read_dir()? {
            read_named_corpus_impl(corpus, entry?.path())?;
        }
    } else {
        let string = fs::read_to_string(path)?;
        corpus.push((
            path.to_owned(),
            to_bytes(string).unwrap_or_else(|| panic!("unable to encode {}", path.display())),
        ));
    }

    Ok(())
}

pub fn read_corpus<P: AsRef<Path>>(path: P) -> io::Result<Vec<Vec<Win1252Char>>> {
    let mut out = Vec::new();
    read_corpus_impl(&mut out, path)?;
    Ok(out)
}

pub fn read_named_corpus<P: AsRef<Path>>(path: P) -> io::Result<Vec<(PathBuf, Vec<Win1252Char>)>> {
    let mut out = Vec::new();
    read_named_corpus_impl(&mut out, path)?;
    Ok(out)
}

#[cfg(test)]
mod tests {
    use std::{fs::File, iter};

    use rand::{prelude::SliceRandom, thread_rng, Rng};

    use super::*;

    #[test]
    fn keys_helloworld() {
        let f = File::open("qwerty.json").unwrap();
        let layout: Layout = serde_json::from_reader(f).unwrap();
        let string = "Hello, WORLD!\n(~1)".to_owned();
        let expected = {
            use TypingEvent::*;
            vec![
                Hold(33),
                Tap(15),
                Release(33),
                Tap(2),
                Tap(18),
                Tap(18),
                Tap(8),
                Tap(27),
                Tap(31),
                Hold(33),
                Tap(1),
                Tap(8),
                Tap(3),
                Tap(18),
                Tap(12),
                Release(33),
                Hold(32),
                Tap(19),
                Tap(31),
                Tap(12),
                Tap(21),
                Release(32),
                Hold(30),
                Tap(16),
                Release(30),
                Hold(32),
                Tap(13),
                Release(32),
            ]
        };

        let actual: Vec<_> = keys(&layout.into(), to_bytes(string).unwrap()).collect();

        assert_eq!(expected, actual);
    }

    #[test]
    fn oneshot_helloworld() {
        let f = File::open("qwerty.json").unwrap();
        let layout: Layout = serde_json::from_reader(f).unwrap();
        let string = "Hello, WORLD!\n(~1)".to_owned();
        let expected = {
            use TypingEvent::*;
            vec![
                Tap(33),
                Tap(15),
                Tap(2),
                Tap(18),
                Tap(18),
                Tap(8),
                Tap(27),
                Tap(31),
                Hold(33),
                Tap(1),
                Tap(8),
                Tap(3),
                Tap(18),
                Tap(12),
                Release(33),
                Hold(32),
                Tap(19),
                Tap(31),
                Tap(12),
                Tap(21),
                Release(32),
                Tap(30),
                Tap(16),
                Tap(32),
                Tap(13),
            ]
        };

        let actual: Vec<_> = oneshot(keys(&layout.into(), to_bytes(string).unwrap())).collect();

        assert_eq!(expected, actual);
    }

    #[test]
    fn type_helloworld() {
        let f = File::open("qwerty.json").unwrap();
        let layout: Layout = serde_json::from_reader(f).unwrap();
        let events = {
            use TypingEvent::*;
            vec![
                Tap(33),
                Tap(15),
                Tap(2),
                Tap(18),
                Tap(18),
                Tap(8),
                Tap(27),
                Tap(31),
                Hold(33),
                Tap(1),
                Tap(8),
                Tap(3),
                Tap(18),
                Tap(12),
                Release(33),
                Hold(32),
                Tap(19),
                Tap(31),
                Tap(12),
                Tap(21),
                Release(32),
                Tap(30),
                Tap(16),
                Tap(32),
                Tap(13),
            ]
        };
        let expected = to_bytes("Hello, WORLD!\n(~1)".to_owned()).unwrap();

        let actual: Vec<_> = type_events(&layout.into(), events).collect();

        assert_eq!(expected, actual);
    }

    #[test]
    fn type_keys() {
        let mut rng = thread_rng();
        let f = File::open("qwerty.json").unwrap();
        let layout: Layout = serde_json::from_reader(f).unwrap();
        let layout: AnnotatedLayout = layout.into();
        let available_chars: Vec<_> = layout
            .char_idx
            .iter()
            .filter_map(|(k, v)| (!v.is_empty()).then_some(k))
            .collect();

        let input_len = rng.gen_range(0..65536);
        let input: Vec<_> = iter::repeat_with(|| *available_chars.choose(&mut rng).unwrap())
            .take(input_len)
            .collect();

        let output: Vec<_> = type_events(&layout, keys(&layout, input.iter().copied())).collect();

        assert_eq!(input, output);
    }

    #[test]
    fn type_oneshot() {
        let mut rng = thread_rng();
        let f = File::open("qwerty.json").unwrap();
        let layout: Layout = serde_json::from_reader(f).unwrap();
        let layout: AnnotatedLayout = layout.into();
        let available_chars: Vec<_> = layout
            .char_idx
            .iter()
            .filter_map(|(k, v)| (!v.is_empty()).then_some(k))
            .collect();

        let input_len = rng.gen_range(0..65536);
        let input: Vec<_> = iter::repeat_with(|| *available_chars.choose(&mut rng).unwrap())
            .take(input_len)
            .collect();

        let output: Vec<_> =
            type_events(&layout, oneshot(keys(&layout, input.iter().copied()))).collect();

        assert_eq!(input, output);
    }
}
