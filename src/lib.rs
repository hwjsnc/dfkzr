/*! # `dfkzr` - Dezimalfestkommazahlrechnung

`dfkzr` provides the type [`UDf64`] which represents unsigned decimal fixed point numbers.

The number of integer and fractional digits is configurable with compile-time generic constants.
Values are represented internally using `u64`.
Arithmetic is done on `u64` or `u128`, depending on the operation.

## Features

- Basic unsigned fixed-point arithmetic
- Wrapper types implementing arithmetic traits for convenience, see [`UDf64Checked`] and [`UDf64Saturating`]
- up to 19 effective decimal digits (integer part and fractional combined)
- `#![no_std]` compatible, no runtime dependencies except `core` (except for some tests)
- No heap allocations
- Written in straightforward safe Rust, accompanied by an extensive automated test suite
- `Display`

# Examples


```
use dfkzr::UDf64;
assert_eq!(
    UDf64::<4, 2>::ONE.checked_add(UDf64::<4, 2>::ONE),
    Ok(UDf64::<4, 2>::try_from_parts(2, 0).unwrap())
);
```

```
use dfkzr::{UDf64, UDf64Checked};
let a: UDf64Checked<6, 3> = UDf64::try_from_parts(1_982, 198).unwrap().into();
let b: UDf64Checked<6, 3> = UDf64::try_from_parts(8_832, 932).unwrap().into();
let c: UDf64Checked<6, 3> = UDf64::try_from_parts(10_815, 130).unwrap().into();
assert_eq!(a + b, Ok(c));
```

```
use dfkzr::udf64;
assert_eq!(udf64!(07, 24).checked_add(udf64!(02, 95)), Ok(udf64!(10, 19)));
```

```
use dfkzr::{udf64, UDf64Saturating};
let a: UDf64Saturating<4, 4> = udf64!(1_483, 0349).into();
let b: UDf64Saturating<4, 4> = udf64!(0_053, 3820).into();
assert_eq!(a * b, UDf64Saturating::<4, 4>::MAX);
```

```
use dfkzr::udf64;
println!("{}", udf64!(0123, 000_789).display()); // prints "123.000789"
```
*/

#![no_std]
#![forbid(unsafe_code)]

#[cfg(all(test, feature = "proptest"))]
const PROPTEST_CASES: u32 = 100_000_000;

mod error;
mod udf64;

pub use error::ArithmeticError;
pub use udf64::{checked::UDf64Checked, saturating::UDf64Saturating, UDf64};
