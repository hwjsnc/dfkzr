# Dezimalfestkommazahlrechnung

## Features

- Basic unsigned fixed-point arithmetic
- Wrapper types implementing arithmetic traits for convenience, see [`UDf64Checked`] and [`UDf64Saturating`]
- up to 19 effective decimal digits (integer part and fractional combined)
- `#![no_std]` compatible, no runtime dependencies except `core` (except for some tests)
- No heap allocations
- Written in straightforward safe Rust, accompanied by an extensive automated test suite
- `Display`

## Planned features:

- signed type
- `TryFrom<&str>`, `FromStr`
- narrowing/widening casts

## Possible future features:

- approximations for:
  - square root, higher order roots
  - exp, log
  - trig functions
- conversion to/from float
- `NonZero` variants
- `u32`-based variants
- localized `Display`/parse

## Non- and anti-goals:

- Choosing precision/width at runtime
- Unchecked/wrapping or panicking API (just use `.unwrap()`)
- Performance: All basic operations are implemented with a handful of operations on `u64` or `u128` (and overflow checks), so performance should be decent. I don't care enough to make benchmarks though.
