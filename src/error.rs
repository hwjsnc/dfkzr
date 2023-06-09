#[cfg(feature = "std")]
extern crate std;

/// Type representing an occured during arithmetic
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ArithmeticError {
    /// Tried to divide by zero
    DivisionByZero,
    /// The result is too large to representable (aka overflow)
    ResultTooLarge,
    /// The result is too small to representable (aka underflow)
    ResultTooSmall,
    /// True result cannot be represented with full precision
    PrecisionLoss,
}

impl core::fmt::Display for ArithmeticError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            ArithmeticError::DivisionByZero => write!(f, "arithmetic error: division by zero"),
            ArithmeticError::ResultTooLarge => write!(f, "arithmetic error: result is too large to be represented"),
            ArithmeticError::ResultTooSmall => write!(f, "arithmetic error: result is too small to be represented"),
            ArithmeticError::PrecisionLoss => write!(f, "arithmetic error: loss of precision"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ArithmeticError {}
