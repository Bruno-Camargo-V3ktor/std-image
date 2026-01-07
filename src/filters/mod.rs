use crate::images::Image;
use std::{error::Error, fmt::Display};

pub mod box_blur;
pub mod edge_detection;
pub mod flip_h;
pub mod flip_v;
pub mod grayscale;
pub mod negative;

// Traits...
pub trait Filter {
    fn apply(&self, image: &mut impl Image) -> Result<(), FilterError>;
}

// Enums...
#[derive(Debug)]
pub enum FilterError {
    InvalidFormat,
}

impl Error for FilterError {
    fn cause(&self) -> Option<&dyn Error> {
        None
    }

    fn description(&self) -> &str {
        match self {
            FilterError::InvalidFormat => "Filter not support for format image",
        }
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl Display for FilterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FilterError::InvalidFormat => {
                write!(f, "FilterError")
            }
        }
    }
}
