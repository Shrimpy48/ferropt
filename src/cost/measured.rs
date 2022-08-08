use std::ops::{Add, Div, Mul, Sub};

use enum_map::{enum_map, EnumMap};
use lazy_static::lazy_static;

use super::CostModel;
use crate::layout::{finger_for_pos, AnnotatedLayout, Digit, Finger, Layer, TypingEvent};

#[derive(Debug, Clone)]
pub struct Model;

impl Default for Model {
    fn default() -> Self {
        Self
    }
}

fn gen_last_used() -> EnumMap<Digit, LastUsedEntry> {
    enum_map! {
        Digit::LeftPinky => LastUsedEntry {
            state: DigitState::Unpressed,
            pos: 10,
        },
        Digit::LeftRing => LastUsedEntry {
            state: DigitState::Unpressed,
            pos: 11,
        },
        Digit::LeftMiddle => LastUsedEntry {
            state: DigitState::Unpressed,
            pos: 12,
        },
        Digit::LeftIndex => LastUsedEntry {
            state: DigitState::Unpressed,
            pos: 13,
        },
        Digit::RightIndex => LastUsedEntry {
            state: DigitState::Unpressed,
            pos: 16,
        },
        Digit::RightMiddle => LastUsedEntry {
            state: DigitState::Unpressed,
            pos: 17,
        },
        Digit::RightRing => LastUsedEntry {
            state: DigitState::Unpressed,
            pos: 18,
        },
        Digit::RightPinky => LastUsedEntry {
            state: DigitState::Unpressed,
            pos: 19,
        },
        Digit::LeftThumb => LastUsedEntry {
            state: DigitState::Unpressed,
            pos: 31,
        },
        Digit::RightThumb => LastUsedEntry {
            state: DigitState::Unpressed,
            pos: 32,
        },
    }
}

fn handle_tap(
    last_used: &mut EnumMap<Digit, LastUsedEntry>,
    cost: &mut f64,
    count: &mut u64,
    pos: u8,
    for_char: bool,
) {
    let digit = DIGIT_FOR_POS[pos];
    match last_used[digit].state {
        DigitState::LastPressed(after) => {
            *cost += COST_FROM_POS[last_used[digit].pos][pos][after as usize]
        }
        _ => *cost += COST_FROM_RESTING[pos],
    }
    advance_counters_etc(last_used, cost, pos);
    if !matches!(last_used[digit].state, DigitState::Held) {
        last_used[digit].state = DigitState::LastPressed(0);
    }
    if for_char {
        *count += 1;
    }
}

fn advance_counters_etc(last_used: &mut EnumMap<Digit, LastUsedEntry>, cost: &mut f64, pos: u8) {
    for e in last_used.values_mut() {
        match e.state {
            DigitState::Held => *cost += COST_OF_HOLDING[e.pos][pos],
            DigitState::LastPressed(ref mut after) => {
                *after += 1;
                if *after as usize >= REPEAT_MAX_T {
                    e.state = DigitState::Unpressed;
                }
            }
            DigitState::Unpressed => {}
        }
    }
}

fn handle_hold(
    last_used: &mut EnumMap<Digit, LastUsedEntry>,
    cost: &mut f64,
    count: &mut u64,
    pos: u8,
) {
    handle_tap(last_used, cost, count, pos, false);
    let digit = DIGIT_FOR_POS[pos];
    last_used[digit] = LastUsedEntry {
        state: DigitState::Held,
        pos,
    };
}

fn handle_release(last_used: &mut EnumMap<Digit, LastUsedEntry>, pos: u8) {
    let digit = DIGIT_FOR_POS[pos];
    last_used[digit] = LastUsedEntry {
        state: DigitState::LastPressed(0),
        pos,
    };
}

impl CostModel for Model {
    fn cost_of_typing(&self, keys: impl Iterator<Item = TypingEvent>) -> (f64, u64) {
        let mut last_used = gen_last_used();

        let mut cost = 0.;
        let mut count = 0;

        for event in keys {
            match event {
                TypingEvent::Tap { pos, for_char } => {
                    handle_tap(&mut last_used, &mut cost, &mut count, pos, for_char)
                }
                TypingEvent::Unknown => {}
                TypingEvent::Hold(pos) => handle_hold(&mut last_used, &mut cost, &mut count, pos),
                TypingEvent::Release(pos) => handle_release(&mut last_used, pos),
            }
        }
        (cost, count)
    }

    fn layout_cost(&self, _layout: &AnnotatedLayout) -> f64 {
        0.
    }
}

#[derive(Debug, Clone, Copy)]
enum DigitState {
    Unpressed,
    Held,
    LastPressed(u8),
}

#[derive(Debug, Clone, Copy)]
struct LastUsedEntry {
    state: DigitState,
    pos: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
struct Millimetre(i16);

impl Mul<i16> for Millimetre {
    type Output = Millimetre;

    fn mul(self, rhs: i16) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl Mul<Millimetre> for i16 {
    type Output = Millimetre;

    fn mul(self, rhs: Millimetre) -> Self::Output {
        Millimetre(self * rhs.0)
    }
}

impl Add for Millimetre {
    type Output = Millimetre;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for Millimetre {
    type Output = Millimetre;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Div for Millimetre {
    type Output = f64;

    fn div(self, rhs: Self) -> Self::Output {
        self.0 as f64 / rhs.0 as f64
    }
}

fn vert_travel(finger: Finger) -> Millimetre {
    Millimetre(match finger {
        Finger::Index => 62,
        Finger::Middle => 72,
        Finger::Ring => 64,
        Finger::Pinky => 49,
        Finger::Thumb => i16::MAX,
    })
}

fn horiz_travel(finger: Finger) -> Millimetre {
    Millimetre(match finger {
        Finger::Index => 37,
        Finger::Thumb => 70,
        _ => i16::MAX,
    })
}

fn finger_strength_cost(finger: Finger) -> f64 {
    match finger {
        Finger::Index => 1.0,
        Finger::Middle => 1.1,
        Finger::Ring => 1.3,
        Finger::Pinky => 1.5,
        Finger::Thumb => 1.2,
    }
}

const DIST_COST: f64 = 2.5;
const HOLD_COST: f64 = 0.5;
const HOLD_SAME_HAND_COST: f64 = 0.5;

const REPEAT_PENALTY: f64 = 3.;
const REPEAT_FALLOFF: f64 = 2.;
const REPEAT_MAX_T: usize = 3;

fn resting_location(digit: Digit) -> (u8, u8) {
    match digit {
        Digit::LeftPinky => (1, 0),
        Digit::LeftRing => (1, 1),
        Digit::LeftMiddle => (1, 2),
        Digit::LeftIndex => (1, 3),
        Digit::RightIndex => (1, 6),
        Digit::RightMiddle => (1, 7),
        Digit::RightRing => (1, 8),
        Digit::RightPinky => (1, 9),
        Digit::LeftThumb => (3, 1),
        Digit::RightThumb => (3, 2),
    }
}

const HORIZ_SEP: Millimetre = Millimetre(18);
const VERT_SEP: Millimetre = Millimetre(17);

lazy_static! {
    static ref RESTING_OFFSET: EnumMap<Finger, Millimetre> = enum_map! {
        Finger::Pinky => Millimetre(-4),
        Finger::Ring => Millimetre(2),
        Finger::Middle => Millimetre(1),
        Finger::Index => Millimetre(-2),
        Finger::Thumb => Millimetre(0),
    };
}

fn dist_from_resting(digit: Digit, (row, col): (u8, u8)) -> (Millimetre, Millimetre) {
    let (rest_row, rest_col) = resting_location(digit);
    let vert_dist = match row.cmp(&rest_row) {
        std::cmp::Ordering::Less => {
            (rest_row - row) as i16 * VERT_SEP - RESTING_OFFSET[digit.finger()]
        }
        std::cmp::Ordering::Equal => Millimetre(0),
        std::cmp::Ordering::Greater => {
            (row - rest_row) as i16 * VERT_SEP + RESTING_OFFSET[digit.finger()]
        }
    };
    let horiz_dist = match col.cmp(&rest_col) {
        std::cmp::Ordering::Less => (rest_col - col) as i16 * HORIZ_SEP,
        std::cmp::Ordering::Equal => Millimetre(0),
        std::cmp::Ordering::Greater => (col - rest_col) as i16 * HORIZ_SEP,
    };
    (horiz_dist, vert_dist)
}

fn dist((from_row, from_col): (u8, u8), (to_row, to_col): (u8, u8)) -> (Millimetre, Millimetre) {
    let vert_dist = match to_row.cmp(&from_row) {
        std::cmp::Ordering::Less => (from_row - to_row) as i16 * VERT_SEP,
        std::cmp::Ordering::Equal => Millimetre(0),
        std::cmp::Ordering::Greater => (to_row - from_row) as i16 * VERT_SEP,
    };
    let horiz_dist = match to_col.cmp(&from_col) {
        std::cmp::Ordering::Less => (from_col - to_col) as i16 * HORIZ_SEP,
        std::cmp::Ordering::Equal => Millimetre(0),
        std::cmp::Ordering::Greater => (to_col - from_col) as i16 * HORIZ_SEP,
    };
    (horiz_dist, vert_dist)
}

fn dist_penalty(finger: Finger, (horiz_dist, vert_dist): (Millimetre, Millimetre)) -> f64 {
    DIST_COST * (horiz_dist / horiz_travel(finger) + vert_dist / vert_travel(finger))
}

fn cost_from_resting(digit: Digit, (row, col): (u8, u8)) -> f64 {
    let dist = dist_from_resting(digit, (row, col));
    finger_strength_cost(digit.finger()) * (1. + dist_penalty(digit.finger(), dist))
}

fn cost_from_pos(digit: Digit, after: u8, from: (u8, u8), to: (u8, u8)) -> f64 {
    if finger_for_pos(to.0, to.1) != digit {
        // Should never be read anyway.
        0.
    } else {
        let dist = dist(from, to);
        finger_strength_cost(digit.finger())
            * (1. + dist_penalty(digit.finger(), dist))
            * REPEAT_PENALTY
            / REPEAT_FALLOFF.powi(after.into())
    }
}

fn cost_of_holding(
    held: Digit,
    _held_pos: (u8, u8),
    pressed: Digit,
    _pressed_pos: (u8, u8),
) -> f64 {
    if held == pressed {
        100.
    } else if held.hand() != pressed.hand() {
        finger_strength_cost(held.finger()) * HOLD_COST
    } else {
        finger_strength_cost(held.finger()) * (HOLD_COST + HOLD_SAME_HAND_COST)
    }
}

lazy_static! {
    static ref DIGIT_FOR_POS: Layer<Digit> = Layer::from_fun(|pos| {
        let r = pos / 10;
        let c = pos % 10;
        finger_for_pos(r, c)
    });
    static ref COST_FROM_RESTING: Layer<f64> = Layer::from_fun(|pos| {
        let r = pos / 10;
        let c = pos % 10;
        let digit = finger_for_pos(r, c);
        cost_from_resting(digit, (r, c))
    });
    static ref COST_FROM_POS: Layer<Layer<[f64; REPEAT_MAX_T]>> = Layer::from_fun(|from_pos| {
        let from_r = from_pos / 10;
        let from_c = from_pos % 10;
        let digit = finger_for_pos(from_r, from_c);
        Layer::from_fun(|to_pos| {
            let to_r = to_pos / 10;
            let to_c = to_pos % 10;
            let mut out = [0.; REPEAT_MAX_T];
            for (after, cost) in (0..).zip(out.iter_mut()) {
                *cost = cost_from_pos(digit, after, (from_r, from_c), (to_r, to_c));
            }
            out
        })
    });
    static ref COST_OF_HOLDING: Layer<Layer<f64>> = Layer::from_fun(|held_pos| {
        let held_r = held_pos / 10;
        let held_c = held_pos % 10;
        let held_digit = finger_for_pos(held_r, held_c);
        Layer::from_fun(|pressed_pos| {
            let pressed_r = pressed_pos / 10;
            let pressed_c = pressed_pos % 10;
            let pressed_digit = finger_for_pos(pressed_r, pressed_c);
            cost_of_holding(
                held_digit,
                (held_r, held_c),
                pressed_digit,
                (pressed_r, pressed_c),
            )
        })
    });
}
