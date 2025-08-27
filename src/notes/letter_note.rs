use std::fmt::Display;
use rand::distr::{Distribution, StandardUniform};

use crispii_errors::{CrispiiError, ImpossibleOperationError, InvalidArgumentError};

use crate::notes::{Modifier, NumberNote, Octave};

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
        match rng.random_range(0..=14) {
            0 => LetterNote::C(Modifier::Flat, rng.random()),
            1 => LetterNote::C(Modifier::Default, rng.random()),
            2 => LetterNote::C(Modifier::Sharp, rng.random()),
            3 => LetterNote::D(Modifier::Flat, rng.random()),
            4 => LetterNote::D(Modifier::Default, rng.random()),
            5 => LetterNote::E(Modifier::Flat, rng.random()),
            6 => LetterNote::E(Modifier::Default, rng.random()),
            7 => LetterNote::F(Modifier::Default, rng.random()),
            8 => LetterNote::F(Modifier::Sharp, rng.random()),
            9 => LetterNote::G(Modifier::Flat, rng.random()),
            10 => LetterNote::G(Modifier::Default, rng.random()),
            11 => LetterNote::A(Modifier::Flat, rng.random()),
            12 => LetterNote::A(Modifier::Default, rng.random()),
            13 => LetterNote::B(Modifier::Flat, rng.random()),
            _ => LetterNote::B(Modifier::Default, rng.random()),
        }
    }
}

impl LetterNote {
    pub fn get_modifier(&self) -> Modifier {
        match self {
            LetterNote::C(modifier, _) => *modifier,
            LetterNote::D(modifier, _) => *modifier,
            LetterNote::E(modifier, _) => *modifier,
            LetterNote::F(modifier, _) => *modifier,
            LetterNote::G(modifier, _) => *modifier,
            LetterNote::A(modifier, _) => *modifier,
            LetterNote::B(modifier, _) => *modifier,
        }
    }

    pub fn get_octave(&self) -> Octave {
        match self {
            LetterNote::C(_, octave) => *octave,
            LetterNote::D(_, octave) => *octave,
            LetterNote::E(_, octave) => *octave,
            LetterNote::F(_, octave) => *octave,
            LetterNote::G(_, octave) => *octave,
            LetterNote::A(_, octave) => *octave,
            LetterNote::B(_, octave) => *octave,
        }
    }

    pub fn try_sharpen(self) -> Result<Self, Box<dyn CrispiiError>> {
        let modifier = self.get_modifier().try_sharpen()?;

        match self {
            LetterNote::C(_, octave) => Ok(LetterNote::C(modifier, octave)),
            LetterNote::D(_, octave) => Ok(LetterNote::D(modifier, octave)),
            LetterNote::E(_, octave) => Ok(LetterNote::E(modifier, octave)),
            LetterNote::F(_, octave) => Ok(LetterNote::F(modifier, octave)),
            LetterNote::G(_, octave) => Ok(LetterNote::G(modifier, octave)),
            LetterNote::A(_, octave) => Ok(LetterNote::A(modifier, octave)),
            LetterNote::B(_, octave) => Ok(LetterNote::B(modifier, octave)),
        }
    }

    pub fn try_flatten(self) -> Result<Self, Box<dyn CrispiiError>> {
        let modifier = self.get_modifier().try_flatten()?;

        match self {
            LetterNote::C(_, octave) => Ok(LetterNote::C(modifier, octave)),
            LetterNote::D(_, octave) => Ok(LetterNote::D(modifier, octave)),
            LetterNote::E(_, octave) => Ok(LetterNote::E(modifier, octave)),
            LetterNote::F(_, octave) => Ok(LetterNote::F(modifier, octave)),
            LetterNote::G(_, octave) => Ok(LetterNote::G(modifier, octave)),
            LetterNote::A(_, octave) => Ok(LetterNote::A(modifier, octave)),
            LetterNote::B(_, octave) => Ok(LetterNote::B(modifier, octave)),
        }
    }

    pub fn try_get_relative_note(&self, number_note: NumberNote) -> Result<LetterNote, Box<dyn CrispiiError>> {
        match self {
            LetterNote::C(modifier, _) => {
                match modifier {
                    Modifier::DoubleFlat | Modifier::DoubleSharp => return Err(Box::new(ImpossibleOperationError::new(format!("{} is not a valid root note!", self).as_str()))),
                    _ => (),
                }
            }
            LetterNote::D(modifier, _) => {
                match modifier {
                    Modifier::DoubleFlat | Modifier::Sharp | Modifier::DoubleSharp => return Err(Box::new(ImpossibleOperationError::new(format!("{} is not a valid root note!", self).as_str()))),
                    _ => (),
                }
            }
            LetterNote::E(modifier, _) => {
                match modifier {
                    Modifier::DoubleFlat | Modifier::Sharp | Modifier::DoubleSharp => return Err(Box::new(ImpossibleOperationError::new(format!("{} is not a valid root note!", self).as_str()))),
                    _ => (),
                }
            }
            LetterNote::F(modifier, _) => {
                match modifier {
                    Modifier::DoubleFlat | Modifier::Flat | Modifier::DoubleSharp => return Err(Box::new(ImpossibleOperationError::new(format!("{} is not a valid root note!", self).as_str()))),
                    _ => (),
                }
            }
            LetterNote::G(modifier, _) => {
                match modifier {
                    Modifier::DoubleFlat | Modifier::Sharp | Modifier::DoubleSharp => return Err(Box::new(ImpossibleOperationError::new(format!("{} is not a valid root note!", self).as_str()))),
                    _ => (),
                }
            }
            LetterNote::A(modifier, _) => {
                match modifier {
                    Modifier::DoubleFlat | Modifier::Sharp | Modifier::DoubleSharp => return Err(Box::new(ImpossibleOperationError::new(format!("{} is not a valid root note!", self).as_str()))),
                    _ => (),
                }
            }
            LetterNote::B(modifier, _) => {
                match modifier {
                    Modifier::DoubleFlat | Modifier::Sharp | Modifier::DoubleSharp => return Err(Box::new(ImpossibleOperationError::new(format!("{} is not a valid root note!", self).as_str()))),
                    _ => (),
                }
            }
        }

        let number_modifier = number_note.get_modifier();

        match number_modifier {
            Modifier::DoubleFlat | Modifier::DoubleSharp => return Err(Box::new(InvalidArgumentError::new("number_note", format!("The {} modifier should only every be returned by this method, not passed to it", number_modifier).as_str()))),
            _ => (),
        }

        let mut relative_note = match self {
            LetterNote::C(modifier, octave) => {
                match number_note {
                    NumberNote::One(_, _) => *self,
                    NumberNote::Two(_, _) => LetterNote::D(*modifier, *octave),
                    NumberNote::Three(_, _) => LetterNote::E(*modifier, *octave),
                    NumberNote::Four(_, _) => LetterNote::F(*modifier, *octave),
                    NumberNote::Five(_, _) => LetterNote::G(*modifier, *octave),
                    NumberNote::Six(_, _) => LetterNote::A(*modifier, *octave),
                    NumberNote::Seven(_, _) => LetterNote::B(*modifier, *octave),
                }
            }
            LetterNote::D(modifier, octave) => {
                match number_note {
                    NumberNote::One(_, _) => *self,
                    NumberNote::Two(_, _) => LetterNote::E(*modifier, *octave),
                    NumberNote::Three(_, _) => LetterNote::F(modifier.try_sharpen().expect("Double sharps have already been factored out"), *octave),
                    NumberNote::Four(_, _) => LetterNote::G(*modifier, *octave),
                    NumberNote::Five(_, _) => LetterNote::A(*modifier, *octave),
                    NumberNote::Six(_, _) => LetterNote::B(*modifier, *octave),
                    NumberNote::Seven(_, _) => LetterNote::C(modifier.try_sharpen().expect("Double sharps have already been factored out"), *octave),
                }
            }
            LetterNote::E(modifier, octave) => {
                match number_note {
                    NumberNote::One(_, _) => *self,
                    NumberNote::Two(_, _) => LetterNote::F(modifier.try_sharpen().expect("Double sharps have already been factored out"), *octave),
                    NumberNote::Three(_, _) => LetterNote::G(modifier.try_sharpen().expect("Double sharps have already been factored out"), *octave),
                    NumberNote::Four(_, _) => LetterNote::A(*modifier, *octave),
                    NumberNote::Five(_, _) => LetterNote::B(*modifier, *octave),
                    NumberNote::Six(_, _) => LetterNote::C(modifier.try_sharpen().expect("Double sharps have already been factored out"), *octave),
                    NumberNote::Seven(_, _) => LetterNote::D(modifier.try_sharpen().expect("Double sharps have already been factored out"), *octave),
                }
            }
            LetterNote::F(modifier, octave) => {
                match number_note {
                    NumberNote::One(_, _) => *self,
                    NumberNote::Two(_, _) => LetterNote::G(*modifier, *octave),
                    NumberNote::Three(_, _) => LetterNote::A(*modifier, *octave),
                    NumberNote::Four(_, _) => LetterNote::B(modifier.try_flatten().expect("Double flats have already been factored out"), *octave),
                    NumberNote::Five(_, _) => LetterNote::C(*modifier, *octave),
                    NumberNote::Six(_, _) => LetterNote::D(*modifier, *octave),
                    NumberNote::Seven(_, _) => LetterNote::E(*modifier, *octave),
                }
            }
            LetterNote::G(modifier, octave) => {
                match number_note {
                    NumberNote::One(_, _) => *self,
                    NumberNote::Two(_, _) => LetterNote::A(*modifier, *octave),
                    NumberNote::Three(_, _) => LetterNote::B(*modifier, *octave),
                    NumberNote::Four(_, _) => LetterNote::C(*modifier, *octave),
                    NumberNote::Five(_, _) => LetterNote::D(*modifier, *octave),
                    NumberNote::Six(_, _) => LetterNote::E(*modifier, *octave),
                    NumberNote::Seven(_, _) => LetterNote::F(modifier.try_sharpen().expect("Double sharps have already been factored out"), *octave),
                }
            }
            LetterNote::A(modifier, octave) => {
                match number_note {
                    NumberNote::One(_, _) => *self,
                    NumberNote::Two(_, _) => LetterNote::B(*modifier, *octave),
                    NumberNote::Three(_, _) => LetterNote::C(modifier.try_sharpen().expect("Double sharps have already been factored out"), *octave),
                    NumberNote::Four(_, _) => LetterNote::D(*modifier, *octave),
                    NumberNote::Five(_, _) => LetterNote::E(*modifier, *octave),
                    NumberNote::Six(_, _) => LetterNote::F(modifier.try_sharpen().expect("Double sharps have already been factored out"), *octave),
                    NumberNote::Seven(_, _) => LetterNote::G(modifier.try_sharpen().expect("Double sharps have already been factored out"), *octave),
                }
            }
            LetterNote::B(modifier, octave) => {
                match number_note {
                    NumberNote::One(_, _) => *self,
                    NumberNote::Two(_, _) => LetterNote::C(modifier.try_sharpen().expect("Double sharps have already been factored out"), *octave),
                    NumberNote::Three(_, _) => LetterNote::D(modifier.try_sharpen().expect("Double sharps have already been factored out"), *octave),
                    NumberNote::Four(_, _) => LetterNote::E(*modifier, *octave),
                    NumberNote::Five(_, _) => LetterNote::F(modifier.try_sharpen().expect("Double sharps have already been factored out"), *octave),
                    NumberNote::Six(_, _) => LetterNote::G(modifier.try_sharpen().expect("Double sharps have already been factored out"), *octave),
                    NumberNote::Seven(_, _) => LetterNote::A(modifier.try_sharpen().expect("Double sharps have already been factored out"), *octave),
                }
            }
        };

        relative_note = match number_modifier {
            Modifier::Flat => relative_note.try_flatten()?,
            Modifier::Sharp => relative_note.try_sharpen()?,
            _ => relative_note,
        };

        Ok(relative_note)
    }
}
