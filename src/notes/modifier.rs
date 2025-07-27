use std::fmt::Display;

use crispii_errors::{CrispiiError, ImpossibleOperationError};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub enum Modifier {
    DoubleFlat,
    Flat,
    #[default]
    Default,
    Sharp,
    DoubleSharp,
}

impl Display for Modifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Modifier::DoubleFlat => write!(f, "𝄫"),
            Modifier::Flat => write!(f, "♭"),
            Modifier::Default => write!(f, ""),
            Modifier::Sharp => write!(f, "♯"),
            Modifier::DoubleSharp => write!(f, "𝄪"),
        }
    }
}

impl Modifier {
    pub fn try_sharpen(self) -> Result<Modifier, Box<dyn CrispiiError>> {
        match self {
            Modifier::DoubleFlat => Ok(Modifier::Flat),
            Modifier::Flat => Ok(Modifier::Default),
            Modifier::Default => Ok(Modifier::Sharp),
            Modifier::Sharp => Ok(Modifier::DoubleSharp),
            Modifier::DoubleSharp => Err(Box::new(ImpossibleOperationError::new(format!("Cannot sharpen a {self} Modifier").as_str()))),
        }
    }

    pub fn try_flatten(self) -> Result<Modifier, Box<dyn CrispiiError>> {
        match self {
            Modifier::DoubleFlat => Err(Box::new(ImpossibleOperationError::new(format!("Cannot flatten a {self} Modifier").as_str()))),
            Modifier::Flat => Ok(Modifier::DoubleFlat),
            Modifier::Default => Ok(Modifier::Flat),
            Modifier::Sharp => Ok(Modifier::Default),
            Modifier::DoubleSharp => Ok(Modifier::Sharp),
        }
    }
}
