use std::fmt::Display;
use rand::distr::{Distribution, StandardUniform};

use crate::notes::{Modifier, Octave};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum NumberNote {
    One(Modifier, Octave),
    Two(Modifier, Octave),
    Three(Modifier, Octave),
    Four(Modifier, Octave),
    Five(Modifier, Octave),
    Six(Modifier, Octave),
    Seven(Modifier, Octave),
}

impl Default for NumberNote {
    fn default() -> Self {
        Self::Four(Modifier::default(), Octave::default())
    }
}

impl Display for NumberNote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NumberNote::One(modifier, octave) => write!(f, "1{modifier}({octave})"),
            NumberNote::Two(modifier, octave) => write!(f, "2{modifier}({octave})"),
            NumberNote::Three(modifier, octave) => write!(f, "3{modifier}({octave})"),
            NumberNote::Four(modifier, octave) => write!(f, "4{modifier}({octave})"),
            NumberNote::Five(modifier, octave) => write!(f, "5{modifier}({octave})"),
            NumberNote::Six(modifier, octave) => write!(f, "6{modifier}({octave})"),
            NumberNote::Seven(modifier, octave) => write!(f, "7{modifier}({octave})"),
        }
    }
}

impl Distribution<NumberNote> for StandardUniform {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> NumberNote {
        match rng.random_range(0..=6) {
            0 => NumberNote::One(rng.random(), rng.random()),
            1 => NumberNote::Two(rng.random(), rng.random()),
            2 => NumberNote::Three(rng.random(), rng.random()),
            3 => NumberNote::Four(rng.random(), rng.random()),
            4 => NumberNote::Five(rng.random(), rng.random()),
            5 => NumberNote::Six(rng.random(), rng.random()),
            _ => NumberNote::Seven(rng.random(), rng.random()),
        }
    }
}
