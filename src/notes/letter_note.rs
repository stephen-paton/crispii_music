use std::fmt::Display;
use rand::distr::{Distribution, StandardUniform};

use crate::notes::{Modifier, Octave};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum LetterNote {
    C(Modifier, Octave),
    D(Modifier, Octave),
    E(Modifier, Octave),
    F(Modifier, Octave),
    G(Modifier, Octave),
    A(Modifier, Octave),
    B(Modifier, Octave),
}

impl Default for LetterNote {
    fn default() -> Self {
        Self::C(Modifier::default(), Octave::default())
    }
}

impl Display for LetterNote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LetterNote::C(modifier, octave) => write!(f, "C{modifier}({octave})"),
            LetterNote::D(modifier, octave) => write!(f, "D{modifier}({octave})"),
            LetterNote::E(modifier, octave) => write!(f, "E{modifier}({octave})"),
            LetterNote::F(modifier, octave) => write!(f, "F{modifier}({octave})"),
            LetterNote::G(modifier, octave) => write!(f, "G{modifier}({octave})"),
            LetterNote::A(modifier, octave) => write!(f, "A{modifier}({octave})"),
            LetterNote::B(modifier, octave) => write!(f, "B{modifier}({octave})"),
        }
    }
}

impl Distribution<LetterNote> for StandardUniform {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> LetterNote {
        match rng.random_range(0..=6) {
            0 => LetterNote::C(rng.random(), rng.random()),
            1 => LetterNote::D(rng.random(), rng.random()),
            2 => LetterNote::E(rng.random(), rng.random()),
            3 => LetterNote::F(rng.random(), rng.random()),
            4 => LetterNote::G(rng.random(), rng.random()),
            5 => LetterNote::A(rng.random(), rng.random()),
            _ => LetterNote::B(rng.random(), rng.random()),
        }
    }
}
