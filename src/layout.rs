use encoding_rs::WINDOWS_1252;
use enum_map::Enum;
use serde::{Deserialize, Serialize};

use std::fmt;
use std::mem::{discriminant, Discriminant};
use std::ops::{Index, IndexMut};
use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

pub const NUM_KEYS: usize = 34;

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
}

lazy_static! {
    static ref A: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("a");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref B: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("b");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref C: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("c");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref D: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("d");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref E: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("e");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref F: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("f");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref G: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("g");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref H: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("h");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref I: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("i");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref J: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("j");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref K: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("k");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref L: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("l");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref M: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("m");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref N: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("n");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref O: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("o");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref P: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("p");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref Q: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("q");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref R: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("r");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref S: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("s");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref T: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("t");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref U: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("u");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref V: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("v");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref W: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("w");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref X: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("x");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref Y: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("y");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref Z: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("z");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref Digit0: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("0");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref Digit1: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("1");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref Digit2: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("2");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref Digit3: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("3");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref Digit4: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("4");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref Digit5: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("5");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref Digit6: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("6");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref Digit7: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("7");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref Digit8: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("8");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref Digit9: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("9");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref Comma: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode(",");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref Dot: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode(".");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref Apostrophe: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("'");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref Semicolon: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode(";");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref Backslash: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("\\");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref Slash: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("/");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref LeftSquareBracket: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("[");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref RightSquareBracket: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("]");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref Hash: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("#");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref Grave: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("`");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref Minus: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("-");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref Equals: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("=");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref Space: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode(" ");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
}
lazy_static! {
    static ref SHIFT_A: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("A");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref SHIFT_B: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("B");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref SHIFT_C: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("C");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref SHIFT_D: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("D");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref SHIFT_E: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("E");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref SHIFT_F: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("F");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref SHIFT_G: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("G");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref SHIFT_H: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("H");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref SHIFT_I: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("I");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref SHIFT_J: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("J");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref SHIFT_K: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("K");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref SHIFT_L: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("L");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref SHIFT_M: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("M");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref SHIFT_N: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("N");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref SHIFT_O: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("O");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref SHIFT_P: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("P");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref SHIFT_Q: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("Q");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref SHIFT_R: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("R");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref SHIFT_S: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("S");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref SHIFT_T: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("T");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref SHIFT_U: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("U");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref SHIFT_V: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("V");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref SHIFT_W: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("W");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref SHIFT_X: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("X");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref SHIFT_Y: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("Y");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref SHIFT_Z: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("Z");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref SHIFT_Digit0: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode(")");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref SHIFT_Digit1: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("!");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref SHIFT_Digit2: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("\"");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref SHIFT_Digit3: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("£");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref SHIFT_Digit4: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("$");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref SHIFT_Digit5: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("%");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref SHIFT_Digit6: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("^");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref SHIFT_Digit7: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("&");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref SHIFT_Digit8: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("*");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref SHIFT_Digit9: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("(");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref SHIFT_Comma: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("<");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref SHIFT_Dot: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode(">");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref SHIFT_Apostrophe: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("@");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref SHIFT_Semicolon: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode(":");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref SHIFT_Backslash: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("|");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref SHIFT_Slash: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("?");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref SHIFT_LeftSquareBracket: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("{");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref SHIFT_RightSquareBracket: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("}");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref SHIFT_Hash: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("~");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref SHIFT_Grave: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("¬");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref SHIFT_Minus: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("_");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref SHIFT_Equals: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode("+");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
    static ref SHIFT_Space: u8 = {
        let (out, _, has_errors) = WINDOWS_1252.encode(" ");
        assert!(!has_errors);
        assert!(out.len() == 1);
        out[0]
    };
}

impl KeyCode {
    fn typed_char(self) -> u8 {
        match self {
            Self::A => *A,
            Self::B => *B,
            Self::C => *C,
            Self::D => *D,
            Self::E => *E,
            Self::F => *F,
            Self::G => *G,
            Self::H => *H,
            Self::I => *I,
            Self::J => *J,
            Self::K => *K,
            Self::L => *L,
            Self::M => *M,
            Self::N => *N,
            Self::O => *O,
            Self::P => *P,
            Self::Q => *Q,
            Self::R => *R,
            Self::S => *S,
            Self::T => *T,
            Self::U => *U,
            Self::V => *V,
            Self::W => *W,
            Self::X => *X,
            Self::Y => *Y,
            Self::Z => *Z,
            Self::Digit0 => *Digit0,
            Self::Digit1 => *Digit1,
            Self::Digit2 => *Digit2,
            Self::Digit3 => *Digit3,
            Self::Digit4 => *Digit4,
            Self::Digit5 => *Digit5,
            Self::Digit6 => *Digit6,
            Self::Digit7 => *Digit7,
            Self::Digit8 => *Digit8,
            Self::Digit9 => *Digit9,
            Self::Comma => *Comma,
            Self::Dot => *Dot,
            Self::Apostrophe => *Apostrophe,
            Self::Semicolon => *Semicolon,
            Self::Backslash => *Backslash,
            Self::Slash => *Slash,
            Self::LeftSquareBracket => *LeftSquareBracket,
            Self::RightSquareBracket => *RightSquareBracket,
            Self::Hash => *Hash,
            Self::Grave => *Grave,
            Self::Minus => *Minus,
            Self::Equals => *Equals,
            Self::Space => *Space,
        }
    }

    fn shifted_char(self) -> u8 {
        match self {
            Self::A => *SHIFT_A,
            Self::B => *SHIFT_B,
            Self::C => *SHIFT_C,
            Self::D => *SHIFT_D,
            Self::E => *SHIFT_E,
            Self::F => *SHIFT_F,
            Self::G => *SHIFT_G,
            Self::H => *SHIFT_H,
            Self::I => *SHIFT_I,
            Self::J => *SHIFT_J,
            Self::K => *SHIFT_K,
            Self::L => *SHIFT_L,
            Self::M => *SHIFT_M,
            Self::N => *SHIFT_N,
            Self::O => *SHIFT_O,
            Self::P => *SHIFT_P,
            Self::Q => *SHIFT_Q,
            Self::R => *SHIFT_R,
            Self::S => *SHIFT_S,
            Self::T => *SHIFT_T,
            Self::U => *SHIFT_U,
            Self::V => *SHIFT_V,
            Self::W => *SHIFT_W,
            Self::X => *SHIFT_X,
            Self::Y => *SHIFT_Y,
            Self::Z => *SHIFT_Z,
            Self::Digit0 => *SHIFT_Digit0,
            Self::Digit1 => *SHIFT_Digit1,
            Self::Digit2 => *SHIFT_Digit2,
            Self::Digit3 => *SHIFT_Digit3,
            Self::Digit4 => *SHIFT_Digit4,
            Self::Digit5 => *SHIFT_Digit5,
            Self::Digit6 => *SHIFT_Digit6,
            Self::Digit7 => *SHIFT_Digit7,
            Self::Digit8 => *SHIFT_Digit8,
            Self::Digit9 => *SHIFT_Digit9,
            Self::Comma => *SHIFT_Comma,
            Self::Dot => *SHIFT_Dot,
            Self::Apostrophe => *SHIFT_Apostrophe,
            Self::Semicolon => *SHIFT_Semicolon,
            Self::Backslash => *SHIFT_Backslash,
            Self::Slash => *SHIFT_Slash,
            Self::LeftSquareBracket => *SHIFT_LeftSquareBracket,
            Self::RightSquareBracket => *SHIFT_RightSquareBracket,
            Self::Hash => *SHIFT_Hash,
            Self::Grave => *SHIFT_Grave,
            Self::Minus => *SHIFT_Minus,
            Self::Equals => *SHIFT_Equals,
            Self::Space => *SHIFT_Space,
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
    pub fn typed_char(self, shifted: bool) -> Option<u8> {
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
