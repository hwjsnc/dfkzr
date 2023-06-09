use super::UDf64;
use crate::ArithmeticError;

use core::ops::{Add, Div, Mul, Sub};

/// Unsigned decimal fixed point value based on [`u64`] with checked arithmetic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
pub struct UDf64Checked<const N1: u8, const N2: u8> {
    value: UDf64<N1, N2>,
}

impl<const N1: u8, const N2: u8> From<UDf64<N1, N2>> for UDf64Checked<N1, N2> {
    fn from(value: UDf64<N1, N2>) -> Self {
        Self { value }
    }
}

impl<const N1: u8, const N2: u8> Into<UDf64<N1, N2>> for UDf64Checked<N1, N2> {
    fn into(self) -> UDf64<N1, N2> {
        self.value
    }
}

impl<const N1: u8, const N2: u8> UDf64Checked<N1, N2> {
    pub const ZERO: Self = Self { value: UDf64::ZERO };
    pub const ONE: Self = Self { value: UDf64::ONE };
    pub const MAX: Self = Self { value: UDf64::ONE };

    /// Checked power
    ///
    /// Returns an error if the result is not exactly representable.
    pub fn pow(self, n: u32) -> Result<Self, ArithmeticError> {
        let mut p = Self::ONE;
        for _ in 0..n {
            // This loop will finish after at most 19 iterations
            p = (p * self)?;
        }
        Ok(p)
    }

    /// Approximate power like [`UDf64::approximate_pow`]
    pub fn approximate_pow(self, n: u32) -> Result<Self, ArithmeticError> {
        self.value.approximate_pow(n).map(Self::from)
    }

    /// Truncating division like [`UDf64::truncating_div`]
    pub fn truncating_div(self, other: Self) -> Result<Self, ArithmeticError> {
        self.value.truncating_div(other.value).map(Self::from)
    }
}

macro_rules! impl_op {
    ($trait_name:ident, $trait_fun:ident, $impl_fun:ident) => {
        impl<const N1: u8, const N2: u8> $trait_name<Self> for UDf64Checked<N1, N2> {
            type Output = Result<Self, ArithmeticError>;

            fn $trait_fun(self, other: Self) -> Self::Output {
                self.value.$impl_fun(other.value).map(Self::from)
            }
        }
    };
}

impl_op!(Add, add, checked_add);
impl_op!(Sub, sub, checked_sub);
impl_op!(Mul, mul, checked_mul);
impl_op!(Div, div, checked_div);

macro_rules! impl_op_for_result {
    ($trait_name:ident, $trait_fun:ident) => {
        impl<const N1: u8, const N2: u8> $trait_name<Result<Self, ArithmeticError>> for UDf64Checked<N1, N2> {
            type Output = Result<Self, ArithmeticError>;

            fn $trait_fun(self, other: Result<Self, ArithmeticError>) -> Self::Output {
                $trait_name::$trait_fun(self, other?)
            }
        }
    };
}

impl_op_for_result!(Add, add);
impl_op_for_result!(Sub, sub);
impl_op_for_result!(Mul, mul);
impl_op_for_result!(Div, div);
