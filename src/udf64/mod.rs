use core::num::NonZeroU64;

use crate::ArithmeticError;

pub mod checked;
pub mod saturating;

#[cfg(test)]
mod test;

#[cfg(all(test, feature = "proptest"))]
mod test_proptest;

/// unsigned decimal fixed point value based on [`u64`]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
pub struct UDf64<const N1: u8, const N2: u8> {
    /// A value of `v` represents the number v * 10**(-N2)
    #[cfg_attr(feature = "proptest", proptest(strategy = "0u64..10u64.pow((N1 + N2) as u32)"))]
    value: u64,
}

impl<const N1: u8, const N2: u8> core::fmt::Debug for UDf64<N1, N2> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "UDf64::<{}, {}> {{ value: {} }}", N1, N2, self.value)
    }
}

impl<const N1: u8, const N2: u8> UDf64<N1, N2> {
    // internally used constants

    /// Check parameters. This _must_ be called from every constructor, including try_from impls!
    /// (The value can be discarded.)
    ///
    /// The limitations arise from u64::MAX being a number with 20 decimal digits.
    pub(crate) const CHECK_PARAMETERS: Result<(), core::convert::Infallible> = if N1 > 19 {
        panic!("N1 (number of digits of the integer part) is too large");
    } else if N2 > 19 {
        panic!("N2 (number of digits of the fractional part) is too large");
    } else if N1 + N2 == 0 {
        panic!("N1+N2 (total number of digits) cannot be 0");
    } else if N1 + N2 > 19 {
        panic!("N1+N2 (number of digits) is too large");
    } else {
        Ok(())
    };

    /// Multiplier that turns a fractional part of 1 (which corresponds to 10**(-N2)) into the value 1.0.
    ///
    /// This multiplier equals 10**(number of digits in the fractional part).
    const FRACT_MULTIPLIER: NonZeroU64 = match NonZeroU64::new(10u64.pow(N2 as u32)) {
        Some(n) => n,
        None => unreachable!(),
    };

    /// The smallest integer that's larger than the biggest representable number.
    ///
    /// The biggest representable number is N1 nines before the decimal point, followed by N2 nines after.
    ///
    /// This differs from [`UDf64::CEIL_REPRESENTABLE_INT:`] by a factor of 10**N2.
    const CEIL_REPRESENTABLE_VALUE: NonZeroU64 = match NonZeroU64::new(10u64.pow((N1 + N2) as u32)) {
        Some(n) => n,
        None => unreachable!(),
    };

    /// The largest representable value.
    ///
    /// N1 nines before the decimal separator, N2 nines after.
    ///
    /// # Examples
    ///
    /// ```
    /// use dfkzr::UDf64;
    /// assert_eq!(UDf64::<6, 3>::MAX.to_parts(), (999_999, 999));
    /// assert_eq!(UDf64::<0, 6>::MAX.to_parts(), (0, 999_999));
    /// assert_eq!(UDf64::<10, 9>::MAX.to_parts(), (9_999_999_999, 999_999_999));
    /// ```
    pub const MAX: Self = match Self::try_from_parts(10u64.pow(N1 as u32) - 1, 10u64.pow(N2 as u32) - 1) {
        Some(n) => n,
        None => unreachable!(),
    };

    /// The value zero, the additive identity.
    pub const ZERO: Self = match Self::try_from_parts(0, 0) {
        Some(n) => n,
        None => unreachable!(),
    };

    /// The value one, the multiplicative identity.
    ///
    /// Not available when `N1 == 0`.
    pub const ONE: Self = match Self::try_from_parts(1, 0) {
        Some(n) => n,
        None if N1 == 0 => panic!("one is not representable when N1 == 0"),
        None => unreachable!(),
    };

    /// Construct a decimal fixed point from integer and fractional part.
    ///
    /// The first argument is the integer part, the second is the fractional part.
    ///
    /// The resulting value is integer.fractional, or integer + fractional * 10**(-N2), assuming that both integer and fractional part are valid.
    ///
    /// # Examples
    ///
    /// ```
    /// use dfkzr::UDf64;
    /// let e = UDf64::<1, 5>::try_from_parts(2, 71828).unwrap();
    /// ```
    ///
    /// ```
    /// use dfkzr::UDf64;
    /// assert_eq!(UDf64::<6, 3>::try_from_parts(1, 0), Some(UDf64::<6, 3>::ONE))
    /// ```
    pub const fn try_from_parts(int: u64, fract: u64) -> Option<Self> {
        let Ok(()) = Self::CHECK_PARAMETERS else { unreachable!() }; // this should be replaced with .from_residual() once stabilized

        if fract < Self::FRACT_MULTIPLIER.get() {
            let Some(n) = int.checked_mul(Self::FRACT_MULTIPLIER.get()) else { return None };
            let Some(value) = n.checked_add(fract) else { return None };
            Self::try_from_internal_representation(value)
        } else {
            None
        }
    }

    /// Construct a decimal fixed point from its internal representation.
    ///
    /// The internal representation is the product of the represented value and 10**N2.
    /// (This is an integer.)
    pub(crate) const fn try_from_internal_representation(value: u64) -> Option<Self> {
        let Ok(()) = Self::CHECK_PARAMETERS else { unreachable!() }; // this should be replaced with .from_residual() once stabilized

        if value < Self::CEIL_REPRESENTABLE_VALUE.get() {
            Some(Self { value })
        } else {
            None
        }
    }

    /// Convert a decimal fraction to its integer and fractional part.
    ///
    /// The first returned value is the integer part, the second is the fractional part multiplied by 10**(-N2).
    ///
    /// # Examples
    ///
    /// ```
    /// use dfkzr::UDf64;
    /// assert_eq!(UDf64::<6, 3>::ONE.to_parts(), (1, 0))
    /// ```
    ///
    /// ```
    /// use dfkzr::UDf64;
    /// assert_eq!(UDf64::<6, 3>::try_from_parts(123_456, 789).unwrap().to_parts(), (123_456, 789))
    /// ```
    pub const fn to_parts(self) -> (u64, u64) {
        let int = self.value / Self::FRACT_MULTIPLIER.get();
        let fract = self.value % Self::FRACT_MULTIPLIER.get();
        (int, fract)
    }
}

impl<const N1: u8, const N2: u8> UDf64<N1, N2> {
    // basic arithmetic

    /// Checked addition
    ///
    /// Returns None when the result is not representable.
    ///
    /// # Examples
    ///
    /// ```
    /// use dfkzr::udf64;
    /// assert_eq!(udf64!(2, 0).checked_add(udf64!(2, 0)), Ok(udf64!(4, 0)));
    /// ```
    ///
    /// ```
    /// use dfkzr::udf64;
    /// assert_eq!(udf64!(3, 14159).checked_add(udf64!(2, 71828)), Ok(udf64!(5, 85987)));
    /// ```
    ///
    /// ```
    /// use dfkzr::{ ArithmeticError, udf64 };
    /// assert_eq!(udf64!(679, 8).checked_add(udf64!(343, 6)), Err(ArithmeticError::ResultTooLarge)); // Result is not representable as UDf64<3, 1>
    /// ```
    pub const fn checked_add(self, other: Self) -> Result<Self, ArithmeticError> {
        match self.value.checked_add(other.value) {
            Some(value) => match Self::try_from_internal_representation(value) {
                Some(value) => Ok(value),
                None => Err(ArithmeticError::ResultTooLarge),
            },
            None => Err(ArithmeticError::ResultTooLarge),
        }
    }

    /// Saturating addition
    ///
    /// When the result exceeds the maximum representable value [`Self::MAX`], returns that value instead.
    ///
    /// # Examples
    ///
    /// ```
    /// use dfkzr::udf64;
    /// assert_eq!(udf64!(2, 0).saturating_add(udf64!(2, 0)), udf64!(4, 0));
    /// ```
    ///
    /// ```
    /// use dfkzr::udf64;
    /// assert_eq!(udf64!(3, 14159).saturating_add(udf64!(2, 71828)), udf64!(5, 85987));
    /// ```
    ///
    /// ```
    /// use dfkzr::udf64;
    /// assert_eq!(udf64!(679, 8).saturating_add(udf64!(343, 6)), udf64!(999, 9)); // True sum exceeds [`UDf64::<3, 1>::MAX`]
    /// ```
    pub const fn saturating_add(self, other: Self) -> Self {
        match self.checked_add(other) {
            Ok(result) => result,
            Err(ArithmeticError::ResultTooLarge) => Self::MAX,
            _ => unreachable!(),
        }
    }

    /// Checked subtraction
    ///
    /// Returns None when the result would be negative.
    ///
    /// # Examples
    ///
    /// ```
    /// use dfkzr::udf64;
    /// assert_eq!(udf64!(128, 84).checked_sub(udf64!(063, 95)), Ok(udf64!(064, 89)));
    /// ```
    ///
    /// ```
    /// use dfkzr::udf64;
    /// assert_eq!(udf64!(3, 14159).checked_sub(udf64!(2, 71828)), Ok(udf64!(0, 42331)));
    /// ```
    ///
    /// ```
    /// use dfkzr::{ ArithmeticError, udf64 };
    /// assert_eq!(udf64!(343, 6).checked_sub(udf64!(679, 8)), Err(ArithmeticError::ResultTooSmall)); // Negative result is not representable
    /// ```
    pub const fn checked_sub(self, other: Self) -> Result<Self, ArithmeticError> {
        match self.value.checked_sub(other.value) {
            Some(value) => match Self::try_from_internal_representation(value) {
                Some(value) => Ok(value),
                None => Err(ArithmeticError::ResultTooLarge),
            },
            None => Err(ArithmeticError::ResultTooSmall),
        }
    }

    /// Saturating subtraction
    ///
    /// When the result is negative, returns 0 instead.
    ///
    /// # Examples
    ///
    /// ```
    /// use dfkzr::udf64;
    /// assert_eq!(udf64!(128, 84).saturating_sub(udf64!(063, 95)), udf64!(064, 89));
    /// ```
    ///
    /// ```
    /// use dfkzr::udf64;
    /// assert_eq!(udf64!(3, 14159).saturating_sub(udf64!(2, 71828)), udf64!(0, 42331));
    /// ```
    ///
    /// ```
    /// use dfkzr::{UDf64, udf64};
    /// assert_eq!(udf64!(343, 6).saturating_sub(udf64!(679, 8)), UDf64::<3, 1>::ZERO); // Negative result is not representable
    /// ```
    pub const fn saturating_sub(self, other: Self) -> Self {
        match self.checked_sub(other) {
            Ok(result) => result,
            Err(ArithmeticError::ResultTooSmall) => Self::ZERO,
            _ => unreachable!(),
        }
    }

    /// Checked multiplication
    ///
    /// Returns None when the result cannot be represented exactly, be it due to overflow or loss of precision.
    ///
    /// # Examples
    ///
    /// ```
    /// use dfkzr::{ ArithmeticError, udf64 };
    /// assert_eq!(udf64!(0038, 90).checked_mul(udf64!(0053, 50)), Ok(udf64!(2081, 15)));
    /// ```
    ///
    /// ```
    /// use dfkzr::{ ArithmeticError, udf64 };
    /// assert_eq!(udf64!(0, 9).checked_mul(udf64!(0, 4)), Err(ArithmeticError::PrecisionLoss));
    /// ```
    ///
    /// ```
    /// use dfkzr::{ ArithmeticError, udf64 };
    /// assert_eq!(udf64!(287, 0).checked_mul(udf64!(843, 4)), Err(ArithmeticError::ResultTooLarge));
    /// ```
    pub const fn checked_mul(self, other: Self) -> Result<Self, ArithmeticError> {
        mul_impl::<N1, N2, /* precision loss is an error = */ true>(self, other)
    }

    /// Truncating multiplication
    ///
    /// Returns None when the result is larger than the maximum representable value.
    /// If the result is smaller than or equal to the maximum representable value, it is rounded down to the nearest representable value (i.e. fractional digits are truncated as needed).
    ///
    /// # Examples
    ///
    /// ```
    /// use dfkzr::{ ArithmeticError, udf64 };
    /// assert_eq!(udf64!(0038, 90).truncating_mul(udf64!(0053, 50)), Ok(udf64!(2081, 15)));
    /// ```
    ///
    /// ```
    /// use dfkzr::{ ArithmeticError, udf64 };
    /// assert_eq!(udf64!(0, 9).truncating_mul(udf64!(0, 4)), Ok(udf64!(0, 3)));
    /// ```
    ///
    /// ```
    /// use dfkzr::{ ArithmeticError, udf64 };
    /// assert_eq!(udf64!(287, 0).truncating_mul(udf64!(843, 4)), Err(ArithmeticError::ResultTooLarge));
    /// ```
    pub const fn truncating_mul(self, other: Self) -> Result<Self, ArithmeticError> {
        mul_impl::<N1, N2, /* precision loss is an error = */ false>(self, other)
    }

    /// Saturating multiplication
    ///
    /// If the result is larger than the maximum representable value, that value is returned instead.
    /// Any fractional digits that cannot be represented are truncated.
    ///
    /// # Examples
    ///
    /// ```
    /// use dfkzr::{ ArithmeticError, udf64 };
    /// assert_eq!(udf64!(0038, 90).saturating_mul(udf64!(0053, 50)), udf64!(2081, 15));
    /// ```
    ///
    /// ```
    /// use dfkzr::{ ArithmeticError, udf64 };
    /// assert_eq!(udf64!(0, 9).saturating_mul(udf64!(0, 4)), udf64!(0, 3));
    /// ```
    ///
    /// ```
    /// use dfkzr::{ ArithmeticError, udf64 };
    /// assert_eq!(udf64!(287, 0).saturating_mul(udf64!(843, 4)), udf64!(999, 9));
    /// ```
    pub const fn saturating_mul(self, other: Self) -> Self {
        match self.truncating_mul(other) {
            Ok(value) => value,
            Err(ArithmeticError::ResultTooLarge) => Self::MAX,
            Err(_) => unreachable!(),
        }
    }

    /// Approximate power
    ///
    /// Returns an approximation of the `n`th power of the decimal fixed-point value, computed by repeatedly calling [`Self::truncating_mul`] in a way that's more efficient than a straightforward loop.
    /// If the true value can be represented exactly, the true value will be returned.
    /// If the true value _cannot_ be represented exactly, the returned value will  _not_ be equal to the true value with truncated digits in general.
    ///
    /// If you need to detect loss of precision, use [`Self::checked_mul`] in a loop.
    ///
    /// Returns an error on overflow.
    ///
    /// This function is not currently available for types with `N1 == 0` (no integer part, all values < 1).
    ///
    /// # Examples
    ///
    /// ```
    /// use dfkzr::{ArithmeticError, udf64};
    /// assert_eq!(udf64!(02, 0).approximate_pow(0), Ok(udf64!(01, 0)));
    /// assert_eq!(udf64!(02, 0).approximate_pow(1), Ok(udf64!(02, 0)));
    /// assert_eq!(udf64!(02, 0).approximate_pow(4), Ok(udf64!(16, 0)));
    /// assert_eq!(udf64!(02, 0).approximate_pow(6), Ok(udf64!(64, 0)));
    /// assert_eq!(udf64!(02, 0).approximate_pow(7), Err(ArithmeticError::ResultTooLarge));
    /// ```
    ///
    /// ```
    /// use dfkzr::{ArithmeticError, udf64};
    /// assert_eq!(udf64!(0, 0).approximate_pow(0), Ok(udf64!(1, 0)));
    /// ```
    ///
    /// ```
    /// use dfkzr::{ArithmeticError, udf64};
    /// assert_eq!(udf64!(1, 10).approximate_pow(2), Ok(udf64!(1, 21)));
    /// assert_eq!(udf64!(1, 10).approximate_pow(3), Ok(udf64!(1, 33))); // Loss of precision is not an error.
    /// assert_eq!(udf64!(1, 10).approximate_pow(5), Ok(udf64!(1, 60))); // True truncated value would be `udf64!(1, 61)`! This exact value is not guaranteed in the future!
    /// assert_eq!(udf64!(1, 10).approximate_pow(25), Err(ArithmeticError::ResultTooLarge));
    /// ```
    ///
    /// ```
    /// use dfkzr::{ArithmeticError, udf64};
    /// assert_eq!(udf64!(0, 5).approximate_pow(6), Ok(udf64!(0, 0))); // Powers of nonzero values can be zero
    /// ```
    ///
    /// ```compile_fail
    /// use dfkzr::{ArithmeticError, UDf64};
    /// let x = UDf64::<0, 3>::try_from_parts(0, 500).unwrap();
    /// let _ = x.approximate_pow(2); // compile error, approximate_pow is not supported
    /// ```
    pub fn approximate_pow(self, mut n: u32) -> Result<Self, ArithmeticError> {
        if n == 0 {
            return Ok(Self::ONE);
        }

        // basic power by squaring algorithm
        let mut a = self;
        let mut b = Self::ONE;
        while n > 1 {
            if n & 1 != 0 {
                b = b.truncating_mul(a)?;
            }
            a = a.truncating_mul(a)?;
            n = n >> 1;
        }
        a.truncating_mul(b)
    }

    /// Saturating power
    ///
    /// Returns an approximation of the `n`th power of the value.
    /// If the result is larger than the maximum representable value, returns that value instead.
    ///
    /// This function is not available for types with `N1 == 0` (no integer part, all values < 1).
    ///
    /// # Examples
    ///
    /// ```
    /// use dfkzr::udf64;
    /// assert_eq!(udf64!(02, 0).saturating_pow(0), udf64!(01, 0));
    /// assert_eq!(udf64!(02, 0).saturating_pow(1), udf64!(02, 0));
    /// assert_eq!(udf64!(02, 0).saturating_pow(4), udf64!(16, 0));
    /// assert_eq!(udf64!(02, 0).saturating_pow(6), udf64!(64, 0));
    /// assert_eq!(udf64!(02, 0).saturating_pow(7), udf64!(99, 9)); // True result is larger than the max representable value
    /// ```
    ///
    /// ```
    /// use dfkzr::udf64;
    /// assert_eq!(udf64!(1, 10).saturating_pow(2), udf64!(1, 21));
    /// assert_eq!(udf64!(1, 10).saturating_pow(3), udf64!(1, 33)); // Loss of precision is not an error.
    /// assert_eq!(udf64!(1, 10).saturating_pow(25), udf64!(9, 99)); // True result is larger than the max representable value
    /// ```
    ///
    /// ```compile_fail
    /// use dfkzr::{ArithmeticError, UDf64};
    /// let x = UDf64::<0, 3>::try_from_parts(0, 500).unwrap();
    /// let _ = x.saturating_pow(0); // compile error, saturating_pow is not supported
    /// ```
    pub fn saturating_pow(self, n: u32) -> Self {
        match self.approximate_pow(n) {
            Ok(value) => value,
            Err(ArithmeticError::ResultTooLarge) => Self::MAX,
            Err(_) => unreachable!(),
        }
    }

    /// Checked division
    ///
    /// Returns None if the divisor is zero or the result cannot be represented exactly.
    ///
    /// # Examples
    ///
    /// ```
    /// use dfkzr::udf64;
    /// assert_eq!(udf64!(8, 53972_12652).checked_div(udf64!(3, 14159_00000)), Ok(udf64!(2, 71828_00000)))
    /// ```
    ///
    /// ```
    /// use dfkzr::{ArithmeticError, udf64};
    /// assert_eq!(udf64!(0, 8).checked_div(udf64!(0, 9)), Err(ArithmeticError::PrecisionLoss))
    /// ```
    ///
    /// ```
    /// use dfkzr::{ArithmeticError, udf64};
    /// assert_eq!(udf64!(85, 6).checked_div(udf64!(00, 2)), Err(ArithmeticError::ResultTooLarge))
    /// ```
    pub fn checked_div(self, other: Self) -> Result<Self, ArithmeticError> {
        div_impl::<N1, N2, /* loss of precision is error = */ true>(self, other)
    }

    /// Truncating division
    ///
    /// Returns None if the divisor is zero or the result exceeds the maximum representable value.
    /// When the result cannot be represented exactly, it is rounded down (truncated) to the nearest representable value.
    ///
    /// ```
    /// use dfkzr::udf64;
    /// assert_eq!(udf64!(8, 53972_12652).truncating_div(udf64!(3, 14159_00000)), Ok(udf64!(2, 71828_00000)))
    /// ```
    ///
    /// ```
    /// use dfkzr::{ ArithmeticError, udf64 };
    /// assert_eq!(udf64!(0, 4).truncating_div(udf64!(0, 7)), Ok(udf64!(0, 5))); // 0.571428... is rounded down to 0.5
    /// ```
    ///
    /// ```
    /// use dfkzr::{ArithmeticError, udf64};
    /// assert_eq!(udf64!(85, 6).truncating_div(udf64!(00, 2)), Err(ArithmeticError::ResultTooLarge))
    /// ```
    pub fn truncating_div(self, other: Self) -> Result<Self, ArithmeticError> {
        div_impl::<N1, N2, /* loss of precision is error = */ false>(self, other)
    }

    /// Saturating division
    ///
    /// Returns None if the divisor is zero.
    /// If the result overflows, the maximum representable value is returned instead.
    /// When the result cannot be represented exactly, it is rounded down (truncated) to the nearest representable value.
    ///
    /// ```
    /// use dfkzr::udf64;
    /// assert_eq!(udf64!(8, 53972_12652).saturating_div(udf64!(3, 14159_00000)), Ok(udf64!(2, 71828_00000)))
    /// ```
    ///
    /// ```
    /// use dfkzr::udf64;
    /// assert_eq!(udf64!(0, 4).saturating_div(udf64!(0, 7)), Ok(udf64!(0, 5))); // 0.571428... is rounded down to 0.5
    /// ```
    ///
    /// ```
    /// use dfkzr::udf64;
    /// assert_eq!(udf64!(85, 6).saturating_div(udf64!(00, 2)), Ok(udf64!(99, 9))); // Overflowing result is replaced with type max
    /// ```
    ///
    /// ```
    /// use dfkzr::{ArithmeticError, udf64};
    /// assert_eq!(udf64!(1, 0).saturating_div(udf64!(0, 0)), Err(ArithmeticError::DivisionByZero)); // Division by zero is still an error
    /// ```
    pub fn saturating_div(self, other: Self) -> Result<Self, ArithmeticError> {
        match self.truncating_div(other) {
            Ok(value) => Ok(value),
            Err(ArithmeticError::ResultTooLarge) => Ok(Self::MAX),
            Err(ArithmeticError::DivisionByZero) => Err(ArithmeticError::DivisionByZero),
            Err(_) => unreachable!(),
        }
    }

    // TODO: saturating_div_by_nonzero

    /// Display the number as a decimal fraction. Returns a value that implements [`core::fmt::Display`].
    ///
    /// # Examples
    ///
    /// ```
    /// use dfkzr::udf64;
    /// assert_eq!(udf64!(2, 71828).display().to_string(), "2.71828")
    /// ```
    ///
    /// ```
    /// use dfkzr::udf64;
    /// assert_eq!(udf64!(123, 456_789).display().to_string(), "123.456789")
    /// ```
    ///
    /// ```
    /// use dfkzr::udf64;
    /// assert_eq!(udf64!(12, 034500).display().to_string(), "12.034500")
    /// ```
    pub fn display(self) -> UDf64Display<N1, N2> {
        UDf64Display { value: self }
    }
}

const fn mul_impl<const N1: u8, const N2: u8, const LOSS_OF_PRECISION_IS_ERROR: bool>(
    x1: UDf64<N1, N2>,
    x2: UDf64<N1, N2>,
) -> Result<UDf64<N1, N2>, ArithmeticError> {
    let v1 = x1.value as u128;
    let v2 = x2.value as u128;
    let product = match v1.checked_mul(v2) {
        Some(n) => n,
        None => unreachable!(), // product of two u64 is representable as u128
    };

    // unfortunately we need to do a bunch of ass conversions here.
    // TODO: revisit this once const conversions are stabilized

    let m = UDf64::<N1, N2>::FRACT_MULTIPLIER.get() as u128;

    if LOSS_OF_PRECISION_IS_ERROR && product % m != 0 {
        return Err(ArithmeticError::PrecisionLoss);
    }

    let value: u128 = product / m;
    if value > (u64::MAX as u128) {
        return Err(ArithmeticError::ResultTooLarge);
    }

    match UDf64::<N1, N2>::try_from_internal_representation(value as u64) {
        Some(value) => Ok(value),
        None => Err(ArithmeticError::ResultTooLarge),
    }
}

const fn div_impl<const N1: u8, const N2: u8, const LOSS_OF_PRECISION_IS_ERROR: bool>(
    dividend: UDf64<N1, N2>,
    divisor: UDf64<N1, N2>,
) -> Result<UDf64<N1, N2>, ArithmeticError> {
    if divisor.value == 0 {
        return Err(ArithmeticError::DivisionByZero);
    }

    // unfortunately we need to do a bunch of ass conversions here.
    // TODO: revisit this once const conversions are stabilized

    let v1 = dividend.value as u128 * UDf64::<N1, N2>::FRACT_MULTIPLIER.get() as u128; // this cannot overflow because both values are < u64::MAX
    let v2 = divisor.value as u128;

    if LOSS_OF_PRECISION_IS_ERROR && v1 % v2 != 0 {
        return Err(ArithmeticError::PrecisionLoss);
    }

    let value: u128 = v1 / v2;
    if value > u64::MAX as u128 {
        return Err(ArithmeticError::ResultTooLarge);
    }

    match UDf64::<N1, N2>::try_from_internal_representation(value as u64) {
        Some(value) => Ok(value),
        None => Err(ArithmeticError::ResultTooLarge),
    }
}

/// Creates compile-time checked [`UDf64`].
/// Digit parameters are determined from the inputs.
///
/// Note that this macro does not allow creation of values with either no integer digits or no fractional digits (parameters `N1` or `N2` equal to zero)
///
/// # Examples
///
/// ```
/// use dfkzr::udf64;
/// let n = udf64!(123, 45);
/// ```
///
/// ```
/// use dfkzr::{UDf64, udf64};
/// assert_eq!(udf64!(001, 00), UDf64::<3, 2>::ONE)
/// ```
///
/// ```
/// use dfkzr::{UDf64, udf64};
/// assert_eq!(udf64!(12_345, 000_010), UDf64::<5, 6>::try_from_parts(12345, 10).unwrap());
/// ```
///
/// ```compile_fail
/// use dfkzr::{UDf64, udf64};
/// let _: UDf64<3, 2> = udf64(12, 34); // mismatched digit parameters
/// ```
#[macro_export]
macro_rules! udf64 {
    ($int:literal, $fract:literal) => {{
        const fn count_digits(s: &str) -> u8 {
            let bytes = s.as_bytes();
            let mut off: usize = 0;
            let mut count: u8 = 0;

            // note: we can't use `.chars()` here because it can't be iterated in const.
            // all digits are ASCII, so we can go byte by byte instead.
            // if we're not dealing with a number, the result of this function doesn't matter anyway.
            while off < bytes.len() {
                let b = bytes[off];
                if b.is_ascii_digit() {
                    count += 1;
                }
                off += 1;
            }
            count
        }

        const N1: u8 = count_digits(stringify!($int));
        const N2: u8 = count_digits(stringify!($fract));
        const NUM: $crate::UDf64<{ N1 }, { N2 }> = match $crate::UDf64::try_from_parts($int, $fract) {
            Some(num) => num,
            None => panic!("invalid arguments to unsigned_checked! macro"),
        };
        NUM
    }};
}

/// Created with [`UDf64::display()`].
pub struct UDf64Display<const N1: u8, const N2: u8> {
    value: UDf64<N1, N2>,
}

impl<const N1: u8, const N2: u8> core::fmt::Display for UDf64Display<N1, N2> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let (int, fract) = self.value.to_parts();
        let digits: usize = N2.into();
        write!(f, "{int}.{fract:0digits$}")
    }
}
