use std::fmt::Display;

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
