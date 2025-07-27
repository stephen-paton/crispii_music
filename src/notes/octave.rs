use std::fmt::Display;

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
