use super::UDf64;
use crate::ArithmeticError;

use core::ops::{Add, Div, Mul, Sub};

/// Unsigned decimal fixed point value based on [`u64`] with saturating arithmetic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
pub struct UDf64Saturating<const N1: u8, const N2: u8> {
    value: UDf64<N1, N2>,
}

impl<const N1: u8, const N2: u8> From<UDf64<N1, N2>> for UDf64Saturating<N1, N2> {
    fn from(value: UDf64<N1, N2>) -> Self {
        Self { value }
    }
}

impl<const N1: u8, const N2: u8> Into<UDf64<N1, N2>> for UDf64Saturating<N1, N2> {
    fn into(self) -> UDf64<N1, N2> {
        self.value
    }
}

impl<const N1: u8, const N2: u8> UDf64Saturating<N1, N2> {
    pub const ZERO: Self = Self { value: UDf64::ZERO };
    pub const ONE: Self = Self { value: UDf64::ONE };
    pub const MAX: Self = Self { value: UDf64::MAX };

    /// Saturating power
    pub fn pow(self, n: u32) -> Self {
        self.value.saturating_pow(n).into()
    }
}

macro_rules! impl_op {
    ($trait_name:ident, $trait_fun:ident, $impl_fun:ident) => {
        impl<const N1: u8, const N2: u8> $trait_name<Self> for UDf64Saturating<N1, N2> {
            type Output = Self;

            fn $trait_fun(self, other: Self) -> Self::Output {
                self.value.$impl_fun(other.value).into()
            }
        }
    };
}

impl_op!(Add, add, saturating_add);
impl_op!(Sub, sub, saturating_sub);
impl_op!(Mul, mul, saturating_mul);

impl<const N1: u8, const N2: u8> Div<Self> for UDf64Saturating<N1, N2> {
    type Output = Result<Self, ArithmeticError>;

    fn div(self, other: Self) -> Self::Output {
        self.value.saturating_div(other.value).map(Self::from)
    }
}
