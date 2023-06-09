// TODO

use super::*;
use proptest::proptest;

proptest! {
    #![proptest_config(proptest::prelude::ProptestConfig::with_cases(crate::PROPTEST_CASES))]

    #[test]
    fn construction_doesnt_crash_19_0(int in 0u64.., fract in 0u64..) {
        let _ = UDf64::<19, 0>::try_from_parts(int, fract);
    }

    #[test]
    fn construction_doesnt_crash_0_19(int in 0u64.., fract in 0u64..) {
        let _ = UDf64::<0, 19>::try_from_parts(int, fract);
    }

    #[test]
    fn construction_doesnt_crash_13_6(int in 0u64.., fract in 0u64..) {
        let _ = UDf64::<13, 6>::try_from_parts(int, fract);
    }

    #[test]
    fn success_10_9(int in 0u64..10u64.pow(10), fract in 0u64..10u64.pow(9)) {
        let _ = UDf64::<10, 9>::try_from_parts(int, fract).unwrap();
    }

    #[test]
    fn success_9_0(int in 0u64..10u64.pow(9)) {
        let _ = UDf64::<9, 0>::try_from_parts(int, 0).unwrap();
    }

    #[test]
    fn success_0_9(fract in 0u64..10u64.pow(9)) {
        let _ = UDf64::<0, 9>::try_from_parts(0, fract).unwrap();
    }

    #[test]
    fn checked_add_is_commutative_6_3(a: UDf64<6, 3>, b: UDf64<6, 3>) {
        assert_eq!(a.checked_add(b), b.checked_add(a))
    }

    #[test]
    fn checked_add_is_associative_6_3(a: UDf64<6, 3>, b: UDf64<6, 3>, c: UDf64<6, 3>) {
        assert_eq!(a.checked_add(b).and_then(|sum| sum.checked_add(c)), b.checked_add(c).and_then(|sum| a.checked_add(sum)))
    }

    #[test]
    fn checked_mul_is_commutative_6_3(a: UDf64<6, 3>, b: UDf64<6, 3>) {
        assert_eq!(a.checked_mul(b), b.checked_mul(a))
    }

    #[test]
    fn checked_mul_and_div_are_inverse_6_3(a: UDf64<6, 3>, b: UDf64<6, 3>) {
        if let Ok(p) = a.checked_mul(b) {
            let zero = UDf64::<6, 3>::ZERO;
            if p == zero {
                assert!(a == zero || b == zero)
            } else {
                assert_eq!(p.checked_div(b), Ok(a));
            }
        }

        if let Ok(q) = a.checked_div(b) {
            assert_eq!(q.checked_mul(b), Ok(a));
        }
    }

    #[test]
    fn saturating_add_is_commutative_6_3(a: UDf64<6, 3>, b: UDf64<6, 3>) {
        assert_eq!(a.saturating_add(b), b.saturating_add(a))
    }

    #[test]
    fn saturating_add_is_associative_6_3(a: UDf64<6, 3>, b: UDf64<6, 3>, c: UDf64<6, 3>) {
        assert_eq!(a.saturating_add(b).saturating_add(c), a.saturating_add(b.saturating_add(c)))
    }

    #[test]
    fn saturating_mul_is_commutative_6_3(a: UDf64<6, 3>, b: UDf64<6, 3>) {
        assert_eq!(a.saturating_mul(b), b.saturating_mul(a))
    }
}
