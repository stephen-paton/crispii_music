use std::fmt::Display;
use rand::distr::{Distribution, StandardUniform};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub enum Octave {
    MinusOne,
    Zero,
    One,
    Two,
    Three,
    #[default]
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl Display for Octave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Octave::MinusOne => write!(f, "-1"),
            Octave::Zero => write!(f, "0"),
            Octave::One => write!(f, "1"),
            Octave::Two => write!(f, "2"),
            Octave::Three => write!(f, "3"),
            Octave::Four => write!(f, "4"),
            Octave::Five => write!(f, "5"),
            Octave::Six => write!(f, "6"),
            Octave::Seven => write!(f, "7"),
            Octave::Eight => write!(f, "8"),
            Octave::Nine => write!(f, "9"),
        }
    }
}

impl Distribution<Octave> for StandardUniform {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Octave {
        match rng.random_range(0..=10) {
            0 => Octave::MinusOne,
            1 => Octave::Zero,
            2 => Octave::One,
            3 => Octave::Two,
            4 => Octave::Three,
            5 => Octave::Four,
            6 => Octave::Five,
            7 => Octave::Six,
            8 => Octave::Seven,
            9 => Octave::Eight,
            _ => Octave::Nine,
        }
    }
}
