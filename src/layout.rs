use serde::ser::{SerializeMap, SerializeTuple};
use serde::{Serialize, Serializer};

use std::fmt;
use std::ops::{Index, IndexMut};

const REQUIRED_CHARS: &str =
    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ,.'\"?!;:\\/<>[]{}()|#@~$&*^+%£`-_= ";

pub const NUM_KEYS: usize = 34;
pub const NUM_LAYERS: usize = 3;

pub const DEFAULT_LAYOUT: Layout = Layout {
    layers: [
        Layer([
            Key::Char('q'),
            Key::Char('w'),
            Key::Char('f'),
            Key::Char('p'),
            Key::Char('b'),
            Key::Char('j'),
            Key::Char('l'),
            Key::Char('u'),
            Key::Char('y'),
            Key::Char(';'),
            Key::Char('a'),
            Key::Char('r'),
            Key::Char('s'),
            Key::Char('t'),
            Key::Char('g'),
            Key::Char('m'),
            Key::Char('n'),
            Key::Char('e'),
            Key::Char('i'),
            Key::Char('o'),
            Key::Char('z'),
            Key::Char('x'),
            Key::Char('c'),
            Key::Char('d'),
            Key::Char('v'),
            Key::Char('k'),
            Key::Char('h'),
            Key::Char(','),
            Key::Char('.'),
            Key::Char('/'),
            Key::Layer(1),
            Key::Char(' '),
            Key::Layer(2),
            Key::Empty,
        ]),
        Layer([
            Key::Char('Q'),
            Key::Char('W'),
            Key::Char('F'),
            Key::Char('P'),
            Key::Char('B'),
            Key::Char('J'),
            Key::Char('L'),
            Key::Char('U'),
            Key::Char('Y'),
            Key::Char(':'),
            Key::Char('A'),
            Key::Char('R'),
            Key::Char('S'),
            Key::Char('T'),
            Key::Char('G'),
            Key::Char('M'),
            Key::Char('N'),
            Key::Char('E'),
            Key::Char('I'),
            Key::Char('O'),
            Key::Char('Z'),
            Key::Char('X'),
            Key::Char('C'),
            Key::Char('D'),
            Key::Char('V'),
            Key::Char('K'),
            Key::Char('H'),
            Key::Char('<'),
            Key::Char('>'),
            Key::Char('?'),
            Key::Empty,
            Key::Empty,
            Key::Empty,
            Key::Empty,
        ]),
        Layer([
            Key::Char('`'),
            Key::Char('$'),
            Key::Char('&'),
            Key::Char('*'),
            Key::Char('^'),
            Key::Char('@'),
            Key::Char('%'),
            Key::Char('+'),
            Key::Char('|'),
            Key::Char('£'),
            Key::Char('\\'),
            Key::Char('\''),
            Key::Char('('),
            Key::Char(')'),
            Key::Char('='),
            Key::Char('#'),
            Key::Char('{'),
            Key::Char('}'),
            Key::Char('-'),
            Key::Char('!'),
            Key::Empty,
            Key::Char('~'),
            Key::Char('_'),
            Key::Char('"'),
            Key::Empty,
            Key::Empty,
            Key::Char('['),
            Key::Char(']'),
            Key::Empty,
            Key::Char('¬'),
            Key::Empty,
            Key::Empty,
            Key::Empty,
            Key::Empty,
        ]),
        // Layer([
        //     Key::Empty,
        //     Key::Char('7'),
        //     Key::Char('8'),
        //     Key::Char('9'),
        //     Key::Empty,
        //     Key::Empty,
        //     Key::Empty,
        //     Key::Empty,
        //     Key::Empty,
        //     Key::Empty,
        //     Key::Char('0'),
        //     Key::Char('1'),
        //     Key::Char('2'),
        //     Key::Char('3'),
        //     Key::Empty,
        //     Key::Empty,
        //     Key::Empty,
        //     Key::Empty,
        //     Key::Empty,
        //     Key::Empty,
        //     Key::Empty,
        //     Key::Char('4'),
        //     Key::Char('5'),
        //     Key::Char('6'),
        //     Key::Empty,
        //     Key::Empty,
        //     Key::Empty,
        //     Key::Empty,
        //     Key::Empty,
        //     Key::Empty,
        // ]),
    ],
};

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

impl Serialize for Key {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
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

impl<T: Serialize> Serialize for Layer<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_tuple(self.0.len())?;
        for elem in &self.0 {
            seq.serialize_element(elem)?;
        }
        seq.end()
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

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct Layout {
    pub(crate) layers: [Layer<Key>; NUM_LAYERS],
}

impl Layout {
    pub(crate) fn iter(&self) -> impl Iterator<Item = &Layer<Key>> {
        self.layers.iter()
    }

    pub fn is_satisfactory(&self) -> bool {
        REQUIRED_CHARS.chars().all(|c| self.has_key(Key::Char(c)))
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

impl Serialize for Layout {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("keyboard", "ferris/sweep")?;
        map.serialize_entry("keymap", "ferropt")?;
        map.serialize_entry("layout", "LAYOUT_split_3x5_2")?;
        map.serialize_entry("layers", &self.layers)?;
        map.end()
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
