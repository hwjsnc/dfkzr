use super::*;

extern crate std;
use std::string::ToString as _;

// TODO: test that construction doesn't crash
#[test]
fn test_instantiation() {}

#[test]
fn test_max() {
    // we also test instantiation of all possible digit lengths

    assert_eq!(UDf64::<0, 1>::MAX.to_parts(), (0, 9));
    assert_eq!(UDf64::<0, 2>::MAX.to_parts(), (0, 99));
    assert_eq!(UDf64::<0, 3>::MAX.to_parts(), (0, 999));
    assert_eq!(UDf64::<0, 4>::MAX.to_parts(), (0, 9999));
    assert_eq!(UDf64::<0, 5>::MAX.to_parts(), (0, 99999));
    assert_eq!(UDf64::<0, 6>::MAX.to_parts(), (0, 999999));
    assert_eq!(UDf64::<0, 7>::MAX.to_parts(), (0, 9999999));
    assert_eq!(UDf64::<0, 8>::MAX.to_parts(), (0, 99999999));
    assert_eq!(UDf64::<0, 9>::MAX.to_parts(), (0, 999999999));
    assert_eq!(UDf64::<0, 10>::MAX.to_parts(), (0, 9999999999));
    assert_eq!(UDf64::<0, 11>::MAX.to_parts(), (0, 99999999999));
    assert_eq!(UDf64::<0, 12>::MAX.to_parts(), (0, 999999999999));
    assert_eq!(UDf64::<0, 13>::MAX.to_parts(), (0, 9999999999999));
    assert_eq!(UDf64::<0, 14>::MAX.to_parts(), (0, 99999999999999));
    assert_eq!(UDf64::<0, 15>::MAX.to_parts(), (0, 999999999999999));
    assert_eq!(UDf64::<0, 16>::MAX.to_parts(), (0, 9999999999999999));
    assert_eq!(UDf64::<0, 17>::MAX.to_parts(), (0, 99999999999999999));
    assert_eq!(UDf64::<0, 18>::MAX.to_parts(), (0, 999999999999999999));
    assert_eq!(UDf64::<0, 19>::MAX.to_parts(), (0, 9999999999999999999));
    assert_eq!(UDf64::<1, 0>::MAX.to_parts(), (9, 0));
    assert_eq!(UDf64::<1, 1>::MAX.to_parts(), (9, 9));
    assert_eq!(UDf64::<1, 2>::MAX.to_parts(), (9, 99));
    assert_eq!(UDf64::<1, 3>::MAX.to_parts(), (9, 999));
    assert_eq!(UDf64::<1, 4>::MAX.to_parts(), (9, 9999));
    assert_eq!(UDf64::<1, 5>::MAX.to_parts(), (9, 99999));
    assert_eq!(UDf64::<1, 6>::MAX.to_parts(), (9, 999999));
    assert_eq!(UDf64::<1, 7>::MAX.to_parts(), (9, 9999999));
    assert_eq!(UDf64::<1, 8>::MAX.to_parts(), (9, 99999999));
    assert_eq!(UDf64::<1, 9>::MAX.to_parts(), (9, 999999999));
    assert_eq!(UDf64::<1, 10>::MAX.to_parts(), (9, 9999999999));
    assert_eq!(UDf64::<1, 11>::MAX.to_parts(), (9, 99999999999));
    assert_eq!(UDf64::<1, 12>::MAX.to_parts(), (9, 999999999999));
    assert_eq!(UDf64::<1, 13>::MAX.to_parts(), (9, 9999999999999));
    assert_eq!(UDf64::<1, 14>::MAX.to_parts(), (9, 99999999999999));
    assert_eq!(UDf64::<1, 15>::MAX.to_parts(), (9, 999999999999999));
    assert_eq!(UDf64::<1, 16>::MAX.to_parts(), (9, 9999999999999999));
    assert_eq!(UDf64::<1, 17>::MAX.to_parts(), (9, 99999999999999999));
    assert_eq!(UDf64::<1, 18>::MAX.to_parts(), (9, 999999999999999999));
    assert_eq!(UDf64::<2, 0>::MAX.to_parts(), (99, 0));
    assert_eq!(UDf64::<2, 1>::MAX.to_parts(), (99, 9));
    assert_eq!(UDf64::<2, 2>::MAX.to_parts(), (99, 99));
    assert_eq!(UDf64::<2, 3>::MAX.to_parts(), (99, 999));
    assert_eq!(UDf64::<2, 4>::MAX.to_parts(), (99, 9999));
    assert_eq!(UDf64::<2, 5>::MAX.to_parts(), (99, 99999));
    assert_eq!(UDf64::<2, 6>::MAX.to_parts(), (99, 999999));
    assert_eq!(UDf64::<2, 7>::MAX.to_parts(), (99, 9999999));
    assert_eq!(UDf64::<2, 8>::MAX.to_parts(), (99, 99999999));
    assert_eq!(UDf64::<2, 9>::MAX.to_parts(), (99, 999999999));
    assert_eq!(UDf64::<2, 10>::MAX.to_parts(), (99, 9999999999));
    assert_eq!(UDf64::<2, 11>::MAX.to_parts(), (99, 99999999999));
    assert_eq!(UDf64::<2, 12>::MAX.to_parts(), (99, 999999999999));
    assert_eq!(UDf64::<2, 13>::MAX.to_parts(), (99, 9999999999999));
    assert_eq!(UDf64::<2, 14>::MAX.to_parts(), (99, 99999999999999));
    assert_eq!(UDf64::<2, 15>::MAX.to_parts(), (99, 999999999999999));
    assert_eq!(UDf64::<2, 16>::MAX.to_parts(), (99, 9999999999999999));
    assert_eq!(UDf64::<2, 17>::MAX.to_parts(), (99, 99999999999999999));
    assert_eq!(UDf64::<3, 0>::MAX.to_parts(), (999, 0));
    assert_eq!(UDf64::<3, 1>::MAX.to_parts(), (999, 9));
    assert_eq!(UDf64::<3, 2>::MAX.to_parts(), (999, 99));
    assert_eq!(UDf64::<3, 3>::MAX.to_parts(), (999, 999));
    assert_eq!(UDf64::<3, 4>::MAX.to_parts(), (999, 9999));
    assert_eq!(UDf64::<3, 5>::MAX.to_parts(), (999, 99999));
    assert_eq!(UDf64::<3, 6>::MAX.to_parts(), (999, 999999));
    assert_eq!(UDf64::<3, 7>::MAX.to_parts(), (999, 9999999));
    assert_eq!(UDf64::<3, 8>::MAX.to_parts(), (999, 99999999));
    assert_eq!(UDf64::<3, 9>::MAX.to_parts(), (999, 999999999));
    assert_eq!(UDf64::<3, 10>::MAX.to_parts(), (999, 9999999999));
    assert_eq!(UDf64::<3, 11>::MAX.to_parts(), (999, 99999999999));
    assert_eq!(UDf64::<3, 12>::MAX.to_parts(), (999, 999999999999));
    assert_eq!(UDf64::<3, 13>::MAX.to_parts(), (999, 9999999999999));
    assert_eq!(UDf64::<3, 14>::MAX.to_parts(), (999, 99999999999999));
    assert_eq!(UDf64::<3, 15>::MAX.to_parts(), (999, 999999999999999));
    assert_eq!(UDf64::<3, 16>::MAX.to_parts(), (999, 9999999999999999));
    assert_eq!(UDf64::<4, 0>::MAX.to_parts(), (9999, 0));
    assert_eq!(UDf64::<4, 1>::MAX.to_parts(), (9999, 9));
    assert_eq!(UDf64::<4, 2>::MAX.to_parts(), (9999, 99));
    assert_eq!(UDf64::<4, 3>::MAX.to_parts(), (9999, 999));
    assert_eq!(UDf64::<4, 4>::MAX.to_parts(), (9999, 9999));
    assert_eq!(UDf64::<4, 5>::MAX.to_parts(), (9999, 99999));
    assert_eq!(UDf64::<4, 6>::MAX.to_parts(), (9999, 999999));
    assert_eq!(UDf64::<4, 7>::MAX.to_parts(), (9999, 9999999));
    assert_eq!(UDf64::<4, 8>::MAX.to_parts(), (9999, 99999999));
    assert_eq!(UDf64::<4, 9>::MAX.to_parts(), (9999, 999999999));
    assert_eq!(UDf64::<4, 10>::MAX.to_parts(), (9999, 9999999999));
    assert_eq!(UDf64::<4, 11>::MAX.to_parts(), (9999, 99999999999));
    assert_eq!(UDf64::<4, 12>::MAX.to_parts(), (9999, 999999999999));
    assert_eq!(UDf64::<4, 13>::MAX.to_parts(), (9999, 9999999999999));
    assert_eq!(UDf64::<4, 14>::MAX.to_parts(), (9999, 99999999999999));
    assert_eq!(UDf64::<4, 15>::MAX.to_parts(), (9999, 999999999999999));
    assert_eq!(UDf64::<5, 0>::MAX.to_parts(), (99999, 0));
    assert_eq!(UDf64::<5, 1>::MAX.to_parts(), (99999, 9));
    assert_eq!(UDf64::<5, 2>::MAX.to_parts(), (99999, 99));
    assert_eq!(UDf64::<5, 3>::MAX.to_parts(), (99999, 999));
    assert_eq!(UDf64::<5, 4>::MAX.to_parts(), (99999, 9999));
    assert_eq!(UDf64::<5, 5>::MAX.to_parts(), (99999, 99999));
    assert_eq!(UDf64::<5, 6>::MAX.to_parts(), (99999, 999999));
    assert_eq!(UDf64::<5, 7>::MAX.to_parts(), (99999, 9999999));
    assert_eq!(UDf64::<5, 8>::MAX.to_parts(), (99999, 99999999));
    assert_eq!(UDf64::<5, 9>::MAX.to_parts(), (99999, 999999999));
    assert_eq!(UDf64::<5, 10>::MAX.to_parts(), (99999, 9999999999));
    assert_eq!(UDf64::<5, 11>::MAX.to_parts(), (99999, 99999999999));
    assert_eq!(UDf64::<5, 12>::MAX.to_parts(), (99999, 999999999999));
    assert_eq!(UDf64::<5, 13>::MAX.to_parts(), (99999, 9999999999999));
    assert_eq!(UDf64::<5, 14>::MAX.to_parts(), (99999, 99999999999999));
    assert_eq!(UDf64::<6, 0>::MAX.to_parts(), (999999, 0));
    assert_eq!(UDf64::<6, 1>::MAX.to_parts(), (999999, 9));
    assert_eq!(UDf64::<6, 2>::MAX.to_parts(), (999999, 99));
    assert_eq!(UDf64::<6, 3>::MAX.to_parts(), (999999, 999));
    assert_eq!(UDf64::<6, 4>::MAX.to_parts(), (999999, 9999));
    assert_eq!(UDf64::<6, 5>::MAX.to_parts(), (999999, 99999));
    assert_eq!(UDf64::<6, 6>::MAX.to_parts(), (999999, 999999));
    assert_eq!(UDf64::<6, 7>::MAX.to_parts(), (999999, 9999999));
    assert_eq!(UDf64::<6, 8>::MAX.to_parts(), (999999, 99999999));
    assert_eq!(UDf64::<6, 9>::MAX.to_parts(), (999999, 999999999));
    assert_eq!(UDf64::<6, 10>::MAX.to_parts(), (999999, 9999999999));
    assert_eq!(UDf64::<6, 11>::MAX.to_parts(), (999999, 99999999999));
    assert_eq!(UDf64::<6, 12>::MAX.to_parts(), (999999, 999999999999));
    assert_eq!(UDf64::<6, 13>::MAX.to_parts(), (999999, 9999999999999));
    assert_eq!(UDf64::<7, 0>::MAX.to_parts(), (9999999, 0));
    assert_eq!(UDf64::<7, 1>::MAX.to_parts(), (9999999, 9));
    assert_eq!(UDf64::<7, 2>::MAX.to_parts(), (9999999, 99));
    assert_eq!(UDf64::<7, 3>::MAX.to_parts(), (9999999, 999));
    assert_eq!(UDf64::<7, 4>::MAX.to_parts(), (9999999, 9999));
    assert_eq!(UDf64::<7, 5>::MAX.to_parts(), (9999999, 99999));
    assert_eq!(UDf64::<7, 6>::MAX.to_parts(), (9999999, 999999));
    assert_eq!(UDf64::<7, 7>::MAX.to_parts(), (9999999, 9999999));
    assert_eq!(UDf64::<7, 8>::MAX.to_parts(), (9999999, 99999999));
    assert_eq!(UDf64::<7, 9>::MAX.to_parts(), (9999999, 999999999));
    assert_eq!(UDf64::<7, 10>::MAX.to_parts(), (9999999, 9999999999));
    assert_eq!(UDf64::<7, 11>::MAX.to_parts(), (9999999, 99999999999));
    assert_eq!(UDf64::<7, 12>::MAX.to_parts(), (9999999, 999999999999));
    assert_eq!(UDf64::<8, 0>::MAX.to_parts(), (99999999, 0));
    assert_eq!(UDf64::<8, 1>::MAX.to_parts(), (99999999, 9));
    assert_eq!(UDf64::<8, 2>::MAX.to_parts(), (99999999, 99));
    assert_eq!(UDf64::<8, 3>::MAX.to_parts(), (99999999, 999));
    assert_eq!(UDf64::<8, 4>::MAX.to_parts(), (99999999, 9999));
    assert_eq!(UDf64::<8, 5>::MAX.to_parts(), (99999999, 99999));
    assert_eq!(UDf64::<8, 6>::MAX.to_parts(), (99999999, 999999));
    assert_eq!(UDf64::<8, 7>::MAX.to_parts(), (99999999, 9999999));
    assert_eq!(UDf64::<8, 8>::MAX.to_parts(), (99999999, 99999999));
    assert_eq!(UDf64::<8, 9>::MAX.to_parts(), (99999999, 999999999));
    assert_eq!(UDf64::<8, 10>::MAX.to_parts(), (99999999, 9999999999));
    assert_eq!(UDf64::<8, 11>::MAX.to_parts(), (99999999, 99999999999));
    assert_eq!(UDf64::<9, 0>::MAX.to_parts(), (999999999, 0));
    assert_eq!(UDf64::<9, 1>::MAX.to_parts(), (999999999, 9));
    assert_eq!(UDf64::<9, 2>::MAX.to_parts(), (999999999, 99));
    assert_eq!(UDf64::<9, 3>::MAX.to_parts(), (999999999, 999));
    assert_eq!(UDf64::<9, 4>::MAX.to_parts(), (999999999, 9999));
    assert_eq!(UDf64::<9, 5>::MAX.to_parts(), (999999999, 99999));
    assert_eq!(UDf64::<9, 6>::MAX.to_parts(), (999999999, 999999));
    assert_eq!(UDf64::<9, 7>::MAX.to_parts(), (999999999, 9999999));
    assert_eq!(UDf64::<9, 8>::MAX.to_parts(), (999999999, 99999999));
    assert_eq!(UDf64::<9, 9>::MAX.to_parts(), (999999999, 999999999));
    assert_eq!(UDf64::<9, 10>::MAX.to_parts(), (999999999, 9999999999));
    assert_eq!(UDf64::<10, 0>::MAX.to_parts(), (9999999999, 0));
    assert_eq!(UDf64::<10, 1>::MAX.to_parts(), (9999999999, 9));
    assert_eq!(UDf64::<10, 2>::MAX.to_parts(), (9999999999, 99));
    assert_eq!(UDf64::<10, 3>::MAX.to_parts(), (9999999999, 999));
    assert_eq!(UDf64::<10, 4>::MAX.to_parts(), (9999999999, 9999));
    assert_eq!(UDf64::<10, 5>::MAX.to_parts(), (9999999999, 99999));
    assert_eq!(UDf64::<10, 6>::MAX.to_parts(), (9999999999, 999999));
    assert_eq!(UDf64::<10, 7>::MAX.to_parts(), (9999999999, 9999999));
    assert_eq!(UDf64::<10, 8>::MAX.to_parts(), (9999999999, 99999999));
    assert_eq!(UDf64::<10, 9>::MAX.to_parts(), (9999999999, 999999999));
    assert_eq!(UDf64::<11, 0>::MAX.to_parts(), (99999999999, 0));
    assert_eq!(UDf64::<11, 1>::MAX.to_parts(), (99999999999, 9));
    assert_eq!(UDf64::<11, 2>::MAX.to_parts(), (99999999999, 99));
    assert_eq!(UDf64::<11, 3>::MAX.to_parts(), (99999999999, 999));
    assert_eq!(UDf64::<11, 4>::MAX.to_parts(), (99999999999, 9999));
    assert_eq!(UDf64::<11, 5>::MAX.to_parts(), (99999999999, 99999));
    assert_eq!(UDf64::<11, 6>::MAX.to_parts(), (99999999999, 999999));
    assert_eq!(UDf64::<11, 7>::MAX.to_parts(), (99999999999, 9999999));
    assert_eq!(UDf64::<11, 8>::MAX.to_parts(), (99999999999, 99999999));
    assert_eq!(UDf64::<12, 0>::MAX.to_parts(), (999999999999, 0));
    assert_eq!(UDf64::<12, 1>::MAX.to_parts(), (999999999999, 9));
    assert_eq!(UDf64::<12, 2>::MAX.to_parts(), (999999999999, 99));
    assert_eq!(UDf64::<12, 3>::MAX.to_parts(), (999999999999, 999));
    assert_eq!(UDf64::<12, 4>::MAX.to_parts(), (999999999999, 9999));
    assert_eq!(UDf64::<12, 5>::MAX.to_parts(), (999999999999, 99999));
    assert_eq!(UDf64::<12, 6>::MAX.to_parts(), (999999999999, 999999));
    assert_eq!(UDf64::<12, 7>::MAX.to_parts(), (999999999999, 9999999));
    assert_eq!(UDf64::<13, 0>::MAX.to_parts(), (9999999999999, 0));
    assert_eq!(UDf64::<13, 1>::MAX.to_parts(), (9999999999999, 9));
    assert_eq!(UDf64::<13, 2>::MAX.to_parts(), (9999999999999, 99));
    assert_eq!(UDf64::<13, 3>::MAX.to_parts(), (9999999999999, 999));
    assert_eq!(UDf64::<13, 4>::MAX.to_parts(), (9999999999999, 9999));
    assert_eq!(UDf64::<13, 5>::MAX.to_parts(), (9999999999999, 99999));
    assert_eq!(UDf64::<13, 6>::MAX.to_parts(), (9999999999999, 999999));
    assert_eq!(UDf64::<14, 0>::MAX.to_parts(), (99999999999999, 0));
    assert_eq!(UDf64::<14, 1>::MAX.to_parts(), (99999999999999, 9));
    assert_eq!(UDf64::<14, 2>::MAX.to_parts(), (99999999999999, 99));
    assert_eq!(UDf64::<14, 3>::MAX.to_parts(), (99999999999999, 999));
    assert_eq!(UDf64::<14, 4>::MAX.to_parts(), (99999999999999, 9999));
    assert_eq!(UDf64::<14, 5>::MAX.to_parts(), (99999999999999, 99999));
    assert_eq!(UDf64::<15, 0>::MAX.to_parts(), (999999999999999, 0));
    assert_eq!(UDf64::<15, 1>::MAX.to_parts(), (999999999999999, 9));
    assert_eq!(UDf64::<15, 2>::MAX.to_parts(), (999999999999999, 99));
    assert_eq!(UDf64::<15, 3>::MAX.to_parts(), (999999999999999, 999));
    assert_eq!(UDf64::<15, 4>::MAX.to_parts(), (999999999999999, 9999));
    assert_eq!(UDf64::<16, 0>::MAX.to_parts(), (9999999999999999, 0));
    assert_eq!(UDf64::<16, 1>::MAX.to_parts(), (9999999999999999, 9));
    assert_eq!(UDf64::<16, 2>::MAX.to_parts(), (9999999999999999, 99));
    assert_eq!(UDf64::<16, 3>::MAX.to_parts(), (9999999999999999, 999));
    assert_eq!(UDf64::<17, 0>::MAX.to_parts(), (99999999999999999, 0));
    assert_eq!(UDf64::<17, 1>::MAX.to_parts(), (99999999999999999, 9));
    assert_eq!(UDf64::<17, 2>::MAX.to_parts(), (99999999999999999, 99));
    assert_eq!(UDf64::<18, 0>::MAX.to_parts(), (999999999999999999, 0));
    assert_eq!(UDf64::<18, 1>::MAX.to_parts(), (999999999999999999, 9));
    assert_eq!(UDf64::<19, 0>::MAX.to_parts(), (9999999999999999999, 0));
}

#[test]
fn test_one() {
    // we also test instantiation of all possible digit lengths

    assert_eq!(UDf64::<1, 0>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<1, 1>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<1, 2>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<1, 3>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<1, 4>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<1, 5>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<1, 6>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<1, 7>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<1, 8>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<1, 9>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<1, 10>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<1, 11>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<1, 12>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<1, 13>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<1, 14>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<1, 15>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<1, 16>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<1, 17>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<1, 18>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<2, 0>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<2, 1>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<2, 2>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<2, 3>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<2, 4>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<2, 5>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<2, 6>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<2, 7>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<2, 8>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<2, 9>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<2, 10>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<2, 11>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<2, 12>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<2, 13>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<2, 14>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<2, 15>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<2, 16>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<2, 17>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<3, 0>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<3, 1>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<3, 2>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<3, 3>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<3, 4>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<3, 5>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<3, 6>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<3, 7>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<3, 8>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<3, 9>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<3, 10>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<3, 11>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<3, 12>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<3, 13>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<3, 14>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<3, 15>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<3, 16>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<4, 0>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<4, 1>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<4, 2>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<4, 3>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<4, 4>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<4, 5>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<4, 6>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<4, 7>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<4, 8>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<4, 9>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<4, 10>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<4, 11>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<4, 12>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<4, 13>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<4, 14>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<4, 15>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<5, 0>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<5, 1>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<5, 2>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<5, 3>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<5, 4>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<5, 5>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<5, 6>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<5, 7>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<5, 8>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<5, 9>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<5, 10>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<5, 11>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<5, 12>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<5, 13>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<5, 14>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<6, 0>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<6, 1>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<6, 2>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<6, 3>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<6, 4>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<6, 5>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<6, 6>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<6, 7>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<6, 8>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<6, 9>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<6, 10>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<6, 11>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<6, 12>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<6, 13>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<7, 0>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<7, 1>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<7, 2>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<7, 3>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<7, 4>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<7, 5>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<7, 6>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<7, 7>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<7, 8>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<7, 9>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<7, 10>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<7, 11>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<7, 12>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<8, 0>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<8, 1>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<8, 2>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<8, 3>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<8, 4>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<8, 5>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<8, 6>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<8, 7>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<8, 8>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<8, 9>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<8, 10>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<8, 11>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<9, 0>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<9, 1>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<9, 2>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<9, 3>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<9, 4>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<9, 5>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<9, 6>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<9, 7>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<9, 8>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<9, 9>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<9, 10>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<10, 0>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<10, 1>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<10, 2>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<10, 3>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<10, 4>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<10, 5>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<10, 6>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<10, 7>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<10, 8>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<10, 9>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<11, 0>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<11, 1>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<11, 2>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<11, 3>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<11, 4>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<11, 5>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<11, 6>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<11, 7>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<11, 8>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<12, 0>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<12, 1>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<12, 2>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<12, 3>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<12, 4>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<12, 5>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<12, 6>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<12, 7>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<13, 0>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<13, 1>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<13, 2>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<13, 3>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<13, 4>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<13, 5>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<13, 6>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<14, 0>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<14, 1>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<14, 2>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<14, 3>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<14, 4>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<14, 5>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<15, 0>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<15, 1>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<15, 2>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<15, 3>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<15, 4>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<16, 0>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<16, 1>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<16, 2>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<16, 3>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<17, 0>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<17, 1>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<17, 2>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<18, 0>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<18, 1>::ONE.to_parts(), (1, 0));
    assert_eq!(UDf64::<19, 0>::ONE.to_parts(), (1, 0));
}

#[test]
fn checked_add_6_3() {
    type T = UDf64<6, 3>;
    assert_eq!(T::ONE.checked_add(T::ONE), Ok(T::try_from_parts(2, 0).unwrap()));
    assert_eq!(
        T::try_from_parts(30, 595).unwrap().checked_add(T::try_from_parts(183, 076).unwrap()),
        Ok(T::try_from_parts(213, 671).unwrap())
    );
    assert_eq!(
        T::try_from_parts(438_292, 743).unwrap().checked_add(T::try_from_parts(173_098, 387).unwrap()),
        Ok(T::try_from_parts(611391, 130).unwrap())
    );
    assert_eq!(
        T::try_from_parts(499_999, 999).unwrap().checked_add(T::try_from_parts(500_000, 000).unwrap()),
        Ok(T::MAX)
    );
    assert_eq!(
        T::try_from_parts(123_456, 789).unwrap().checked_add(T::try_from_parts(876_543, 210).unwrap()),
        Ok(T::MAX)
    );
    assert_eq!(
        T::try_from_parts(123_456, 789).unwrap().checked_add(T::try_from_parts(876_543, 211).unwrap()),
        Err(ArithmeticError::ResultTooLarge)
    );
    assert_eq!(T::MAX.checked_add(T::MAX), Err(ArithmeticError::ResultTooLarge));
}

#[test]
fn checked_add_4_0() {
    type T = UDf64<4, 0>;
    assert_eq!(T::ONE.checked_add(T::ONE), Ok(T::try_from_parts(2, 0).unwrap()));
    assert_eq!(
        T::try_from_parts(382, 0).unwrap().checked_add(T::try_from_parts(8192, 0).unwrap()),
        Ok(T::try_from_parts(8574, 0).unwrap())
    );
    assert_eq!(
        T::try_from_parts(94, 0).unwrap().checked_add(T::try_from_parts(79, 0).unwrap()),
        Ok(T::try_from_parts(173, 0).unwrap())
    );
    assert_eq!(T::try_from_parts(1234, 0).unwrap().checked_add(T::try_from_parts(8765, 0).unwrap()), Ok(T::MAX));
    assert_eq!(
        T::try_from_parts(1234, 0).unwrap().checked_add(T::try_from_parts(8766, 0).unwrap()),
        Err(ArithmeticError::ResultTooLarge)
    );
    assert_eq!(T::MAX.checked_add(T::MAX), Err(ArithmeticError::ResultTooLarge));
}

#[test]
fn checked_add_0_4() {
    type T = UDf64<0, 4>;
    assert_eq!(
        T::try_from_parts(0, 1).unwrap().checked_add(T::try_from_parts(0, 1).unwrap()),
        Ok(T::try_from_parts(0, 2).unwrap())
    );
    assert_eq!(
        T::try_from_parts(0, 382).unwrap().checked_add(T::try_from_parts(0, 8192).unwrap()),
        Ok(T::try_from_parts(0, 8574).unwrap())
    );
    assert_eq!(
        T::try_from_parts(0, 94).unwrap().checked_add(T::try_from_parts(0, 79).unwrap()),
        Ok(T::try_from_parts(0, 173).unwrap())
    );
    assert_eq!(T::try_from_parts(0, 1234).unwrap().checked_add(T::try_from_parts(0, 8765).unwrap()), Ok(T::MAX));
    assert_eq!(
        T::try_from_parts(0, 1234).unwrap().checked_add(T::try_from_parts(0, 8766).unwrap()),
        Err(ArithmeticError::ResultTooLarge)
    );
    assert_eq!(T::MAX.checked_add(T::MAX), Err(ArithmeticError::ResultTooLarge));
}

#[test]
fn checked_add_10_9() {
    type T = UDf64<10, 9>;
    assert_eq!(T::ONE.checked_add(T::ONE), Ok(T::try_from_parts(2, 0).unwrap()));
    assert_eq!(
        T::try_from_parts(3_239_830, 829_012_595)
            .unwrap()
            .checked_add(T::try_from_parts(182_940, 040_109_201).unwrap()),
        Ok(T::try_from_parts(3_422_770, 869_121_796).unwrap())
    );
    assert_eq!(
        T::try_from_parts(389_380, 0).unwrap().checked_add(T::try_from_parts(83_710, 0).unwrap()),
        Ok(T::try_from_parts(473_090, 0).unwrap())
    );
    assert_eq!(
        T::try_from_parts(0, 389_380).unwrap().checked_add(T::try_from_parts(0, 83_710).unwrap()),
        Ok(T::try_from_parts(0, 473_090).unwrap())
    );
    assert_eq!(
        T::try_from_parts(328, 843_800_000)
            .unwrap()
            .checked_add(T::try_from_parts(84, 192_000_000).unwrap()),
        Ok(T::try_from_parts(413, 035_800_000).unwrap())
    );
    assert_eq!(
        T::try_from_parts(23_898_120, 389_201_002)
            .unwrap()
            .checked_add(T::try_from_parts(19_837_808, 099_138_710).unwrap()),
        Ok(T::try_from_parts(43_735_928, 488_339_712).unwrap())
    );
    assert_eq!(
        T::try_from_parts(1_234_567_890, 123_456_789)
            .unwrap()
            .checked_add(T::try_from_parts(8_765_432_109, 876_543_210).unwrap()),
        Ok(T::MAX)
    );
    assert_eq!(
        T::try_from_parts(1_234_567_890, 123_456_789)
            .unwrap()
            .checked_add(T::try_from_parts(8_765_432_109, 876_543_211).unwrap()),
        Err(ArithmeticError::ResultTooLarge),
    );
    assert_eq!(T::MAX.checked_add(T::try_from_parts(0, 1).unwrap()), Err(ArithmeticError::ResultTooLarge));
    assert_eq!(T::MAX.checked_add(T::MAX), Err(ArithmeticError::ResultTooLarge));
}

#[test]
fn saturating_add_6_3() {
    type T = UDf64<6, 3>;
    assert_eq!(T::ONE.saturating_add(T::ONE), T::try_from_parts(2, 0).unwrap());
    assert_eq!(
        T::try_from_parts(30, 595).unwrap().saturating_add(T::try_from_parts(183, 076).unwrap()),
        T::try_from_parts(213, 671).unwrap()
    );
    assert_eq!(
        T::try_from_parts(438_292, 743)
            .unwrap()
            .saturating_add(T::try_from_parts(173_098, 387).unwrap()),
        T::try_from_parts(611391, 130).unwrap()
    );
    assert_eq!(
        T::try_from_parts(499_999, 999)
            .unwrap()
            .saturating_add(T::try_from_parts(500_000, 000).unwrap()),
        T::MAX
    );
    assert_eq!(
        T::try_from_parts(123_456, 789)
            .unwrap()
            .saturating_add(T::try_from_parts(876_543, 210).unwrap()),
        T::MAX
    );
    assert_eq!(
        T::try_from_parts(123_456, 789)
            .unwrap()
            .saturating_add(T::try_from_parts(876_543, 211).unwrap()),
        T::MAX
    );
    assert_eq!(T::MAX.saturating_add(T::MAX), T::MAX);
}

#[test]
fn saturating_add_4_0() {
    type T = UDf64<4, 0>;
    assert_eq!(T::ONE.saturating_add(T::ONE), T::try_from_parts(2, 0).unwrap());
    assert_eq!(
        T::try_from_parts(382, 0).unwrap().saturating_add(T::try_from_parts(8192, 0).unwrap()),
        T::try_from_parts(8574, 0).unwrap()
    );
    assert_eq!(
        T::try_from_parts(94, 0).unwrap().saturating_add(T::try_from_parts(79, 0).unwrap()),
        T::try_from_parts(173, 0).unwrap()
    );
    assert_eq!(T::try_from_parts(1234, 0).unwrap().saturating_add(T::try_from_parts(8765, 0).unwrap()), T::MAX);
    assert_eq!(T::try_from_parts(1234, 0).unwrap().saturating_add(T::try_from_parts(8766, 0).unwrap()), T::MAX);
    assert_eq!(T::MAX.saturating_add(T::MAX), T::MAX);
}

#[test]
fn saturating_add_0_4() {
    type T = UDf64<0, 4>;
    assert_eq!(
        T::try_from_parts(0, 1).unwrap().saturating_add(T::try_from_parts(0, 1).unwrap()),
        T::try_from_parts(0, 2).unwrap()
    );
    assert_eq!(
        T::try_from_parts(0, 382).unwrap().saturating_add(T::try_from_parts(0, 8192).unwrap()),
        T::try_from_parts(0, 8574).unwrap()
    );
    assert_eq!(
        T::try_from_parts(0, 94).unwrap().saturating_add(T::try_from_parts(0, 79).unwrap()),
        T::try_from_parts(0, 173).unwrap()
    );
    assert_eq!(T::try_from_parts(0, 1234).unwrap().saturating_add(T::try_from_parts(0, 8765).unwrap()), T::MAX);
    assert_eq!(T::try_from_parts(0, 1234).unwrap().saturating_add(T::try_from_parts(0, 8766).unwrap()), T::MAX);
    assert_eq!(T::MAX.saturating_add(T::MAX), T::MAX);
}

#[test]
fn saturating_add_10_9() {
    type T = UDf64<10, 9>;
    assert_eq!(T::ONE.saturating_add(T::ONE), T::try_from_parts(2, 0).unwrap());
    assert_eq!(
        T::try_from_parts(3_239_830, 829_012_595)
            .unwrap()
            .saturating_add(T::try_from_parts(182_940, 040_109_201).unwrap()),
        T::try_from_parts(3_422_770, 869_121_796).unwrap()
    );
    assert_eq!(
        T::try_from_parts(389_380, 0).unwrap().saturating_add(T::try_from_parts(83_710, 0).unwrap()),
        T::try_from_parts(473_090, 0).unwrap()
    );
    assert_eq!(
        T::try_from_parts(0, 389_380).unwrap().saturating_add(T::try_from_parts(0, 83_710).unwrap()),
        T::try_from_parts(0, 473_090).unwrap()
    );
    assert_eq!(
        T::try_from_parts(328, 843_800_000)
            .unwrap()
            .saturating_add(T::try_from_parts(84, 192_000_000).unwrap()),
        T::try_from_parts(413, 035_800_000).unwrap()
    );
    assert_eq!(
        T::try_from_parts(23_898_120, 389_201_002)
            .unwrap()
            .saturating_add(T::try_from_parts(19_837_808, 099_138_710).unwrap()),
        T::try_from_parts(43_735_928, 488_339_712).unwrap()
    );
    assert_eq!(
        T::try_from_parts(1_234_567_890, 123_456_789)
            .unwrap()
            .saturating_add(T::try_from_parts(8_765_432_109, 876_543_210).unwrap()),
        T::MAX
    );
    assert_eq!(
        T::try_from_parts(1_234_567_890, 123_456_789)
            .unwrap()
            .saturating_add(T::try_from_parts(8_765_432_109, 876_543_211).unwrap()),
        T::MAX,
    );
    assert_eq!(T::MAX.saturating_add(T::try_from_parts(0, 1).unwrap()), T::MAX);
    assert_eq!(T::MAX.saturating_add(T::MAX), T::MAX);
}

#[test]
fn saturating_add_4_2() {
    type T = UDf64<4, 2>;
    assert_eq!(T::ONE.saturating_div(T::ONE).unwrap(), T::ONE);
    assert_eq!(
        T::try_from_parts(1234, 56).unwrap().saturating_div(T::try_from_parts(2, 0).unwrap()).unwrap(),
        T::try_from_parts(617, 28).unwrap()
    );
    assert_eq!(
        T::try_from_parts(1234, 56).unwrap().saturating_div(T::try_from_parts(0, 13).unwrap()).unwrap(),
        T::try_from_parts(9496, 61).unwrap()
    );
    assert_eq!(
        T::try_from_parts(1234, 56).unwrap().saturating_div(T::try_from_parts(0, 12).unwrap()).unwrap(),
        T::MAX
    );
}

#[test]
fn checked_sub_6_3() {
    type T = UDf64<6, 3>;
    assert_eq!(
        T::try_from_parts(142_849, 405).unwrap().checked_sub(T::try_from_parts(83_849, 871).unwrap()),
        Ok(T::try_from_parts(58999, 534).unwrap())
    );
    assert_eq!(
        T::try_from_parts(698_829, 382).unwrap().checked_sub(T::try_from_parts(428_940, 243).unwrap()),
        Ok(T::try_from_parts(269889, 139).unwrap())
    );
    assert_eq!(
        T::try_from_parts(8_382, 019).unwrap().checked_sub(T::try_from_parts(172, 110).unwrap()),
        Ok(T::try_from_parts(8209, 909).unwrap())
    );
    assert_eq!(
        T::try_from_parts(83_849, 871).unwrap().checked_sub(T::try_from_parts(142_849, 40).unwrap()),
        Err(ArithmeticError::ResultTooSmall)
    );
    assert_eq!(
        T::try_from_parts(428_940, 243).unwrap().checked_sub(T::try_from_parts(698_829, 382).unwrap()),
        Err(ArithmeticError::ResultTooSmall)
    );
    assert_eq!(
        T::try_from_parts(172, 110).unwrap().checked_sub(T::try_from_parts(8_382, 019).unwrap()),
        Err(ArithmeticError::ResultTooSmall)
    );
    assert_eq!(T::ONE.checked_sub(T::ONE), Ok(T::ZERO));
    assert_eq!(T::MAX.checked_sub(T::ZERO), Ok(T::MAX));
    assert_eq!(T::MAX.checked_sub(T::MAX), Ok(T::ZERO));
    assert_eq!(T::ZERO.checked_sub(T::MAX), Err(ArithmeticError::ResultTooSmall));
}

#[test]
fn checked_sub_4_0() {
    type T = UDf64<4, 0>;
    assert_eq!(
        T::try_from_parts(3291, 0).unwrap().checked_sub(T::try_from_parts(1921, 0).unwrap()),
        Ok(T::try_from_parts(1370, 0).unwrap())
    );
    assert_eq!(
        T::try_from_parts(9878, 0).unwrap().checked_sub(T::try_from_parts(4009, 0).unwrap()),
        Ok(T::try_from_parts(5869, 0).unwrap())
    );
    assert_eq!(
        T::try_from_parts(6218, 0).unwrap().checked_sub(T::try_from_parts(21, 0).unwrap()),
        Ok(T::try_from_parts(6197, 0).unwrap())
    );
    assert_eq!(
        T::try_from_parts(1921, 0).unwrap().checked_sub(T::try_from_parts(3291, 0).unwrap()),
        Err(ArithmeticError::ResultTooSmall)
    );
    assert_eq!(
        T::try_from_parts(4009, 0).unwrap().checked_sub(T::try_from_parts(9878, 0).unwrap()),
        Err(ArithmeticError::ResultTooSmall)
    );
    assert_eq!(
        T::try_from_parts(21, 0).unwrap().checked_sub(T::try_from_parts(6218, 0).unwrap()),
        Err(ArithmeticError::ResultTooSmall)
    );
    assert_eq!(T::ONE.checked_sub(T::ONE), Ok(T::ZERO));
    assert_eq!(T::MAX.checked_sub(T::ZERO), Ok(T::MAX));
    assert_eq!(T::MAX.checked_sub(T::MAX), Ok(T::ZERO));
    assert_eq!(T::ZERO.checked_sub(T::MAX), Err(ArithmeticError::ResultTooSmall));
}

#[test]
fn checked_sub_0_4() {
    type T = UDf64<0, 4>;
    assert_eq!(
        T::try_from_parts(0, 3291).unwrap().checked_sub(T::try_from_parts(0, 1921).unwrap()),
        Ok(T::try_from_parts(0, 1370).unwrap())
    );
    assert_eq!(
        T::try_from_parts(0, 9878).unwrap().checked_sub(T::try_from_parts(0, 4009).unwrap()),
        Ok(T::try_from_parts(0, 5869).unwrap())
    );
    assert_eq!(
        T::try_from_parts(0, 6218).unwrap().checked_sub(T::try_from_parts(0, 21).unwrap()),
        Ok(T::try_from_parts(0, 6197).unwrap())
    );
    assert_eq!(
        T::try_from_parts(0, 1921).unwrap().checked_sub(T::try_from_parts(0, 3291).unwrap()),
        Err(ArithmeticError::ResultTooSmall)
    );
    assert_eq!(
        T::try_from_parts(0, 4009).unwrap().checked_sub(T::try_from_parts(0, 9878).unwrap()),
        Err(ArithmeticError::ResultTooSmall)
    );
    assert_eq!(
        T::try_from_parts(0, 21).unwrap().checked_sub(T::try_from_parts(0, 6218).unwrap()),
        Err(ArithmeticError::ResultTooSmall)
    );
    assert_eq!(
        T::try_from_parts(0, 5000).unwrap().checked_sub(T::try_from_parts(0, 5000).unwrap()),
        Ok(T::ZERO)
    );
    assert_eq!(T::MAX.checked_sub(T::ZERO), Ok(T::MAX));
    assert_eq!(T::MAX.checked_sub(T::MAX), Ok(T::ZERO));
    assert_eq!(T::ZERO.checked_sub(T::MAX), Err(ArithmeticError::ResultTooSmall));
}

#[test]
fn checked_sub_10_9() {
    type T = UDf64<10, 9>;
    assert_eq!(
        T::try_from_parts(1_234_807_234, 087_423_081)
            .unwrap()
            .checked_sub(T::try_from_parts(738_471_284, 981_409_824).unwrap()),
        Ok(T::try_from_parts(496_335_949, 106_013_257).unwrap())
    );
    assert_eq!(
        T::try_from_parts(89_743, 048_719_323)
            .unwrap()
            .checked_sub(T::try_from_parts(0, 000_017_834).unwrap()),
        Ok(T::try_from_parts(89_743, 048_701_489).unwrap())
    );
    assert_eq!(
        T::try_from_parts(182_937_438, 934_512_347)
            .unwrap()
            .checked_sub(T::try_from_parts(130_489_489, 992_829_188).unwrap()),
        Ok(T::try_from_parts(52_447_948, 941_683_159).unwrap())
    );
    assert_eq!(
        T::try_from_parts(1_234_567_890, 123_456_789)
            .unwrap()
            .checked_sub(T::try_from_parts(1_234_567_890, 123_456_788).unwrap()),
        Ok(T::try_from_parts(0, 1).unwrap())
    );
    assert_eq!(
        T::try_from_parts(1_000_000_000, 000_000_000)
            .unwrap()
            .checked_sub(T::try_from_parts(0, 1).unwrap()),
        Ok(T::try_from_parts(999_999_999, 999_999_999).unwrap())
    );
    assert_eq!(T::ONE.checked_sub(T::ONE), Ok(T::ZERO));
    assert_eq!(T::MAX.checked_sub(T::ZERO), Ok(T::MAX));
    assert_eq!(T::MAX.checked_sub(T::MAX), Ok(T::ZERO));
    assert_eq!(T::ZERO.checked_sub(T::MAX), Err(ArithmeticError::ResultTooSmall));
}

#[test]
fn saturating_sub_6_3() {
    type T = UDf64<6, 3>;
    assert_eq!(
        T::try_from_parts(142_849, 405).unwrap().saturating_sub(T::try_from_parts(83_849, 871).unwrap()),
        T::try_from_parts(58999, 534).unwrap()
    );
    assert_eq!(
        T::try_from_parts(698_829, 382)
            .unwrap()
            .saturating_sub(T::try_from_parts(428_940, 243).unwrap()),
        T::try_from_parts(269889, 139).unwrap()
    );
    assert_eq!(
        T::try_from_parts(8_382, 019).unwrap().saturating_sub(T::try_from_parts(172, 110).unwrap()),
        T::try_from_parts(8209, 909).unwrap()
    );
    assert_eq!(
        T::try_from_parts(83_849, 871).unwrap().saturating_sub(T::try_from_parts(142_849, 40).unwrap()),
        T::ZERO
    );
    assert_eq!(
        T::try_from_parts(428_940, 243)
            .unwrap()
            .saturating_sub(T::try_from_parts(698_829, 382).unwrap()),
        T::ZERO
    );
    assert_eq!(
        T::try_from_parts(172, 110).unwrap().saturating_sub(T::try_from_parts(8_382, 019).unwrap()),
        T::ZERO
    );
    assert_eq!(T::ONE.saturating_sub(T::ONE), T::ZERO);
    assert_eq!(T::MAX.saturating_sub(T::ZERO), T::MAX);
    assert_eq!(T::MAX.saturating_sub(T::MAX), T::ZERO);
    assert_eq!(T::ZERO.saturating_sub(T::MAX), T::ZERO);
}

#[test]
fn saturating_sub_4_0() {
    type T = UDf64<4, 0>;
    assert_eq!(
        T::try_from_parts(3291, 0).unwrap().saturating_sub(T::try_from_parts(1921, 0).unwrap()),
        T::try_from_parts(1370, 0).unwrap()
    );
    assert_eq!(
        T::try_from_parts(9878, 0).unwrap().saturating_sub(T::try_from_parts(4009, 0).unwrap()),
        T::try_from_parts(5869, 0).unwrap()
    );
    assert_eq!(
        T::try_from_parts(6218, 0).unwrap().saturating_sub(T::try_from_parts(21, 0).unwrap()),
        T::try_from_parts(6197, 0).unwrap()
    );
    assert_eq!(T::try_from_parts(1921, 0).unwrap().saturating_sub(T::try_from_parts(3291, 0).unwrap()), T::ZERO);
    assert_eq!(T::try_from_parts(4009, 0).unwrap().saturating_sub(T::try_from_parts(9878, 0).unwrap()), T::ZERO);
    assert_eq!(T::try_from_parts(21, 0).unwrap().saturating_sub(T::try_from_parts(6218, 0).unwrap()), T::ZERO);
    assert_eq!(T::ONE.saturating_sub(T::ONE), T::ZERO);
    assert_eq!(T::MAX.saturating_sub(T::ZERO), T::MAX);
    assert_eq!(T::MAX.saturating_sub(T::MAX), T::ZERO);
    assert_eq!(T::ZERO.saturating_sub(T::MAX), T::ZERO);
}

#[test]
fn saturating_sub_0_4() {
    type T = UDf64<0, 4>;
    assert_eq!(
        T::try_from_parts(0, 3291).unwrap().saturating_sub(T::try_from_parts(0, 1921).unwrap()),
        T::try_from_parts(0, 1370).unwrap()
    );
    assert_eq!(
        T::try_from_parts(0, 9878).unwrap().saturating_sub(T::try_from_parts(0, 4009).unwrap()),
        T::try_from_parts(0, 5869).unwrap()
    );
    assert_eq!(
        T::try_from_parts(0, 6218).unwrap().saturating_sub(T::try_from_parts(0, 21).unwrap()),
        T::try_from_parts(0, 6197).unwrap()
    );
    assert_eq!(T::try_from_parts(0, 1921).unwrap().saturating_sub(T::try_from_parts(0, 3291).unwrap()), T::ZERO);
    assert_eq!(T::try_from_parts(0, 4009).unwrap().saturating_sub(T::try_from_parts(0, 9878).unwrap()), T::ZERO);
    assert_eq!(T::try_from_parts(0, 21).unwrap().saturating_sub(T::try_from_parts(0, 6218).unwrap()), T::ZERO);
    assert_eq!(T::try_from_parts(0, 5000).unwrap().saturating_sub(T::try_from_parts(0, 5000).unwrap()), T::ZERO);
    assert_eq!(T::MAX.saturating_sub(T::ZERO), T::MAX);
    assert_eq!(T::MAX.saturating_sub(T::MAX), T::ZERO);
    assert_eq!(T::ZERO.saturating_sub(T::MAX), T::ZERO);
}

#[test]
fn saturating_sub_10_9() {
    type T = UDf64<10, 9>;
    assert_eq!(
        T::try_from_parts(1_234_807_234, 087_423_081)
            .unwrap()
            .saturating_sub(T::try_from_parts(738_471_284, 981_409_824).unwrap()),
        T::try_from_parts(496_335_949, 106_013_257).unwrap()
    );
    assert_eq!(
        T::try_from_parts(89_743, 048_719_323)
            .unwrap()
            .saturating_sub(T::try_from_parts(0, 000_017_834).unwrap()),
        T::try_from_parts(89_743, 048_701_489).unwrap()
    );
    assert_eq!(
        T::try_from_parts(182_937_438, 934_512_347)
            .unwrap()
            .saturating_sub(T::try_from_parts(130_489_489, 992_829_188).unwrap()),
        T::try_from_parts(52_447_948, 941_683_159).unwrap()
    );
    assert_eq!(
        T::try_from_parts(1_234_567_890, 123_456_789)
            .unwrap()
            .saturating_sub(T::try_from_parts(1_234_567_890, 123_456_788).unwrap()),
        T::try_from_parts(0, 1).unwrap()
    );
    assert_eq!(
        T::try_from_parts(1_000_000_000, 000_000_000)
            .unwrap()
            .saturating_sub(T::try_from_parts(0, 1).unwrap()),
        T::try_from_parts(999_999_999, 999_999_999).unwrap()
    );
    assert_eq!(T::ONE.saturating_sub(T::ONE), T::ZERO);
    assert_eq!(T::MAX.saturating_sub(T::ZERO), T::MAX);
    assert_eq!(T::MAX.saturating_sub(T::MAX), T::ZERO);
    assert_eq!(T::ZERO.saturating_sub(T::MAX), T::ZERO);
}

#[test]
fn checked_mul_6_3() {
    type T = UDf64<6, 3>;
    assert_eq!(
        T::try_from_parts(2, 500).unwrap().checked_mul(T::try_from_parts(9, 0).unwrap()),
        Ok(T::try_from_parts(22, 500).unwrap())
    );
    assert_eq!(
        T::try_from_parts(2387, 100).unwrap().checked_mul(T::try_from_parts(352, 360).unwrap()),
        Ok(T::try_from_parts(841_118, 556).unwrap())
    );
    assert_eq!(
        T::try_from_parts(2, 500).unwrap().checked_mul(T::try_from_parts(1, 300).unwrap()),
        Ok(T::try_from_parts(3, 250).unwrap())
    );
    assert_eq!(
        T::try_from_parts(2, 580).unwrap().checked_mul(T::try_from_parts(1, 350).unwrap()),
        Ok(T::try_from_parts(3, 483).unwrap())
    );
    assert_eq!(
        T::try_from_parts(193, 920).unwrap().checked_mul(T::try_from_parts(292, 650).unwrap()),
        Ok(T::try_from_parts(56_750, 688).unwrap())
    );
    assert_eq!(
        T::try_from_parts(0, 120).unwrap().checked_mul(T::try_from_parts(0, 036).unwrap()),
        Err(ArithmeticError::PrecisionLoss)
    );
    assert_eq!(
        T::try_from_parts(1686, 200).unwrap().checked_mul(T::try_from_parts(5_932, 400).unwrap()),
        Err(ArithmeticError::ResultTooLarge)
    );
    assert_eq!(T::ONE.checked_mul(T::ONE), Ok(T::ONE));
    assert_eq!(T::ONE.checked_mul(T::ZERO), Ok(T::ZERO));
    assert_eq!(T::MAX.checked_mul(T::ZERO), Ok(T::ZERO));
    assert_eq!(T::ONE.checked_mul(T::MAX), Ok(T::MAX));
    assert!(T::MAX.checked_mul(T::MAX).is_err());
}

#[test]
fn checked_mul_4_0() {
    type T = UDf64<4, 0>;
    assert_eq!(
        T::try_from_parts(11, 0).unwrap().checked_mul(T::try_from_parts(23, 0).unwrap()),
        Ok(T::try_from_parts(253, 0).unwrap())
    );
    assert_eq!(
        T::try_from_parts(2136, 0).unwrap().checked_mul(T::try_from_parts(3, 0).unwrap()),
        Ok(T::try_from_parts(6408, 0).unwrap())
    );
    assert_eq!(
        T::try_from_parts(99, 0).unwrap().checked_mul(T::try_from_parts(73, 0).unwrap()),
        Ok(T::try_from_parts(7227, 0).unwrap())
    );
    assert_eq!(T::ONE.checked_mul(T::ONE), Ok(T::ONE));
    assert_eq!(T::ONE.checked_mul(T::ZERO), Ok(T::ZERO));
    assert_eq!(T::MAX.checked_mul(T::ZERO), Ok(T::ZERO));
    assert_eq!(T::ONE.checked_mul(T::MAX), Ok(T::MAX));
    assert_eq!(T::MAX.checked_mul(T::MAX), Err(ArithmeticError::ResultTooLarge));
}

#[test]
fn checked_mul_0_4() {
    type T = UDf64<0, 4>;
    assert_eq!(
        T::try_from_parts(0, 1100).unwrap().checked_mul(T::try_from_parts(0, 2300).unwrap()),
        Ok(T::try_from_parts(0, 0253).unwrap())
    );
    assert_eq!(
        T::try_from_parts(0, 2130).unwrap().checked_mul(T::try_from_parts(0, 3000).unwrap()),
        Ok(T::try_from_parts(0, 0639).unwrap())
    );
    assert_eq!(
        T::try_from_parts(0, 9900).unwrap().checked_mul(T::try_from_parts(0, 7300).unwrap()),
        Ok(T::try_from_parts(0, 7227).unwrap())
    );
    assert_eq!(
        T::try_from_parts(0, 9990).unwrap().checked_mul(T::try_from_parts(0, 1000).unwrap()),
        Ok(T::try_from_parts(0, 0999).unwrap())
    );
    assert_eq!(T::MAX.checked_mul(T::ZERO), Ok(T::ZERO));
    assert_eq!(T::MAX.checked_mul(T::MAX), Err(ArithmeticError::PrecisionLoss));
}

#[test]
fn checked_mul_10_9() {
    type T = UDf64<10, 9>;
    assert_eq!(
        T::try_from_parts(2, 718_280_000)
            .unwrap()
            .checked_mul(T::try_from_parts(3, 141_500_000).unwrap()),
        Ok(T::try_from_parts(8, 539_476_620).unwrap())
    );
    assert_eq!(
        T::try_from_parts(2, 718_280_000)
            .unwrap()
            .checked_mul(T::try_from_parts(3, 141_590_000).unwrap()),
        Err(ArithmeticError::PrecisionLoss),
    );
    assert_eq!(
        T::try_from_parts(72_850, 038_520_000)
            .unwrap()
            .checked_mul(T::try_from_parts(92_820, 994_600_000).unwrap()),
        Ok(T::try_from_parts(6_762_013_032, 074_711_992).unwrap())
    );
    assert_eq!(
        T::try_from_parts(172_850, 038_520_000)
            .unwrap()
            .checked_mul(T::try_from_parts(92_820, 994_600_000).unwrap()),
        Err(ArithmeticError::ResultTooLarge),
    );
    assert_eq!(
        T::try_from_parts(72_850, 038_520_000)
            .unwrap()
            .checked_mul(T::try_from_parts(92_820, 994_660_000).unwrap()),
        Err(ArithmeticError::PrecisionLoss),
    );
    assert!(T::try_from_parts(172_850, 038_520_000)
        .unwrap()
        .checked_mul(T::try_from_parts(92_820, 994_660_000).unwrap())
        .is_err());
    assert_eq!(T::ONE.checked_mul(T::ONE), Ok(T::ONE));
    assert_eq!(T::ONE.checked_mul(T::ZERO), Ok(T::ZERO));
    assert_eq!(T::MAX.checked_mul(T::ZERO), Ok(T::ZERO));
    assert_eq!(T::ONE.checked_mul(T::MAX), Ok(T::MAX));
    assert!(T::MAX.checked_mul(T::MAX).is_err());
}

#[test]
fn truncating_mul_6_3() {
    type T = UDf64<6, 3>;
    assert_eq!(
        T::try_from_parts(2, 500).unwrap().truncating_mul(T::try_from_parts(9, 0).unwrap()),
        Ok(T::try_from_parts(22, 500).unwrap())
    );
    assert_eq!(
        T::try_from_parts(2387, 100).unwrap().truncating_mul(T::try_from_parts(352, 360).unwrap()),
        Ok(T::try_from_parts(841_118, 556).unwrap())
    );
    assert_eq!(
        T::try_from_parts(2, 500).unwrap().truncating_mul(T::try_from_parts(1, 300).unwrap()),
        Ok(T::try_from_parts(3, 250).unwrap())
    );
    assert_eq!(
        T::try_from_parts(2, 580).unwrap().truncating_mul(T::try_from_parts(1, 350).unwrap()),
        Ok(T::try_from_parts(3, 483).unwrap())
    );
    assert_eq!(
        T::try_from_parts(193, 920).unwrap().truncating_mul(T::try_from_parts(292, 650).unwrap()),
        Ok(T::try_from_parts(56_750, 688).unwrap())
    );
    assert_eq!(
        T::try_from_parts(0, 120).unwrap().truncating_mul(T::try_from_parts(0, 036).unwrap()),
        Ok(T::try_from_parts(0, 004).unwrap())
    );
    assert_eq!(
        T::try_from_parts(1686, 200).unwrap().truncating_mul(T::try_from_parts(5_932, 400).unwrap()),
        Err(ArithmeticError::ResultTooLarge)
    );
    assert_eq!(T::ONE.truncating_mul(T::ONE), Ok(T::ONE));
    assert_eq!(T::ONE.truncating_mul(T::ZERO), Ok(T::ZERO));
    assert_eq!(T::MAX.truncating_mul(T::ZERO), Ok(T::ZERO));
    assert_eq!(T::ONE.truncating_mul(T::MAX), Ok(T::MAX));
    assert_eq!(T::MAX.truncating_mul(T::MAX), Err(ArithmeticError::ResultTooLarge));
}

#[test]
fn truncating_mul_4_0() {
    type T = UDf64<4, 0>;
    assert_eq!(
        T::try_from_parts(11, 0).unwrap().truncating_mul(T::try_from_parts(23, 0).unwrap()),
        Ok(T::try_from_parts(253, 0).unwrap())
    );
    assert_eq!(
        T::try_from_parts(2136, 0).unwrap().truncating_mul(T::try_from_parts(3, 0).unwrap()),
        Ok(T::try_from_parts(6408, 0).unwrap())
    );
    assert_eq!(
        T::try_from_parts(99, 0).unwrap().truncating_mul(T::try_from_parts(73, 0).unwrap()),
        Ok(T::try_from_parts(7227, 0).unwrap())
    );
    assert_eq!(T::ONE.truncating_mul(T::ONE), Ok(T::ONE));
    assert_eq!(T::ONE.truncating_mul(T::ZERO), Ok(T::ZERO));
    assert_eq!(T::MAX.truncating_mul(T::ZERO), Ok(T::ZERO));
    assert_eq!(T::ONE.truncating_mul(T::MAX), Ok(T::MAX));
    assert_eq!(T::MAX.truncating_mul(T::MAX), Err(ArithmeticError::ResultTooLarge));
}

#[test]
fn truncating_mul_0_4() {
    type T = UDf64<0, 4>;
    assert_eq!(
        T::try_from_parts(0, 1100).unwrap().truncating_mul(T::try_from_parts(0, 2300).unwrap()),
        Ok(T::try_from_parts(0, 0253).unwrap())
    );
    assert_eq!(
        T::try_from_parts(0, 2130).unwrap().truncating_mul(T::try_from_parts(0, 3000).unwrap()),
        Ok(T::try_from_parts(0, 0639).unwrap())
    );
    assert_eq!(
        T::try_from_parts(0, 9900).unwrap().truncating_mul(T::try_from_parts(0, 7300).unwrap()),
        Ok(T::try_from_parts(0, 7227).unwrap())
    );
    assert_eq!(
        T::try_from_parts(0, 0099).unwrap().truncating_mul(T::try_from_parts(0, 0053).unwrap()),
        Ok(T::ZERO)
    );
    assert_eq!(
        T::try_from_parts(0, 9990).unwrap().truncating_mul(T::try_from_parts(0, 1000).unwrap()),
        Ok(T::try_from_parts(0, 0999).unwrap())
    );
    assert_eq!(T::MAX.truncating_mul(T::ZERO), Ok(T::ZERO));
    assert_eq!(T::MAX.truncating_mul(T::MAX), Ok(T::try_from_parts(0, 9998).unwrap()));
}

#[test]
fn truncating_mul_10_9() {
    type T = UDf64<10, 9>;
    assert_eq!(
        T::try_from_parts(2, 718_280_000)
            .unwrap()
            .truncating_mul(T::try_from_parts(3, 141_500_000).unwrap()),
        Ok(T::try_from_parts(8, 539_476_620).unwrap())
    );
    assert_eq!(
        T::try_from_parts(2, 718_280_000)
            .unwrap()
            .truncating_mul(T::try_from_parts(3, 141_590_000).unwrap()),
        Ok(T::try_from_parts(8, 539_721_265).unwrap()),
    );
    assert_eq!(
        T::try_from_parts(72_850, 038_520_000)
            .unwrap()
            .truncating_mul(T::try_from_parts(92_820, 994_600_000).unwrap()),
        Ok(T::try_from_parts(6_762_013_032, 074_711_992).unwrap())
    );
    assert_eq!(
        T::try_from_parts(172_850, 038_520_000)
            .unwrap()
            .truncating_mul(T::try_from_parts(92_820, 994_600_000).unwrap()),
        Err(ArithmeticError::ResultTooLarge),
    );
    assert_eq!(
        T::try_from_parts(72_850, 038_520_000)
            .unwrap()
            .truncating_mul(T::try_from_parts(92_820, 994_660_000).unwrap()),
        Ok(T::try_from_parts(6_762_013_036, 445_714_303).unwrap()),
    );
    assert_eq!(
        T::try_from_parts(172_850, 038_520_000)
            .unwrap()
            .truncating_mul(T::try_from_parts(92_820, 994_660_000).unwrap()),
        Err(ArithmeticError::ResultTooLarge)
    );
    assert_eq!(
        T::try_from_parts(0, 000_001_000)
            .unwrap()
            .truncating_mul(T::try_from_parts(0, 000_004_000).unwrap()),
        Ok(T::ZERO)
    );
    assert_eq!(T::ONE.truncating_mul(T::ONE), Ok(T::ONE));
    assert_eq!(T::ONE.truncating_mul(T::ZERO), Ok(T::ZERO));
    assert_eq!(T::MAX.truncating_mul(T::ZERO), Ok(T::ZERO));
    assert_eq!(T::ONE.truncating_mul(T::MAX), Ok(T::MAX));
    assert_eq!(T::MAX.truncating_mul(T::MAX), Err(ArithmeticError::ResultTooLarge));
}

#[test]
fn saturating_mul_6_3() {
    type T = UDf64<6, 3>;
    assert_eq!(
        T::try_from_parts(2, 500).unwrap().saturating_mul(T::try_from_parts(9, 0).unwrap()),
        T::try_from_parts(22, 500).unwrap()
    );
    assert_eq!(
        T::try_from_parts(2387, 100).unwrap().saturating_mul(T::try_from_parts(352, 360).unwrap()),
        T::try_from_parts(841_118, 556).unwrap()
    );
    assert_eq!(
        T::try_from_parts(2, 500).unwrap().saturating_mul(T::try_from_parts(1, 300).unwrap()),
        T::try_from_parts(3, 250).unwrap()
    );
    assert_eq!(
        T::try_from_parts(2, 580).unwrap().saturating_mul(T::try_from_parts(1, 350).unwrap()),
        T::try_from_parts(3, 483).unwrap()
    );
    assert_eq!(
        T::try_from_parts(193, 920).unwrap().saturating_mul(T::try_from_parts(292, 650).unwrap()),
        T::try_from_parts(56_750, 688).unwrap()
    );
    assert_eq!(
        T::try_from_parts(0, 120).unwrap().saturating_mul(T::try_from_parts(0, 036).unwrap()),
        T::try_from_parts(0, 004).unwrap()
    );
    assert_eq!(
        T::try_from_parts(1686, 200).unwrap().saturating_mul(T::try_from_parts(5_932, 400).unwrap()),
        T::MAX
    );
    assert_eq!(T::ONE.saturating_mul(T::ONE), T::ONE);
    assert_eq!(T::ONE.saturating_mul(T::ZERO), T::ZERO);
    assert_eq!(T::MAX.saturating_mul(T::ZERO), T::ZERO);
    assert_eq!(T::ONE.saturating_mul(T::MAX), T::MAX);
    assert_eq!(T::MAX.saturating_mul(T::MAX), T::MAX);
}

#[test]
fn saturating_mul_4_0() {
    type T = UDf64<4, 0>;
    assert_eq!(
        T::try_from_parts(11, 0).unwrap().saturating_mul(T::try_from_parts(23, 0).unwrap()),
        T::try_from_parts(253, 0).unwrap()
    );
    assert_eq!(
        T::try_from_parts(2136, 0).unwrap().saturating_mul(T::try_from_parts(3, 0).unwrap()),
        T::try_from_parts(6408, 0).unwrap()
    );
    assert_eq!(
        T::try_from_parts(99, 0).unwrap().saturating_mul(T::try_from_parts(73, 0).unwrap()),
        T::try_from_parts(7227, 0).unwrap()
    );
    assert_eq!(T::ONE.saturating_mul(T::ONE), T::ONE);
    assert_eq!(T::ONE.saturating_mul(T::ZERO), T::ZERO);
    assert_eq!(T::MAX.saturating_mul(T::ZERO), T::ZERO);
    assert_eq!(T::ONE.saturating_mul(T::MAX), T::MAX);
    assert_eq!(T::MAX.saturating_mul(T::MAX), T::MAX);
}

#[test]
fn saturating_mul_0_4() {
    type T = UDf64<0, 4>;
    assert_eq!(
        T::try_from_parts(0, 1100).unwrap().saturating_mul(T::try_from_parts(0, 2300).unwrap()),
        T::try_from_parts(0, 0253).unwrap()
    );
    assert_eq!(
        T::try_from_parts(0, 2130).unwrap().saturating_mul(T::try_from_parts(0, 3000).unwrap()),
        T::try_from_parts(0, 0639).unwrap()
    );
    assert_eq!(
        T::try_from_parts(0, 9900).unwrap().saturating_mul(T::try_from_parts(0, 7300).unwrap()),
        T::try_from_parts(0, 7227).unwrap()
    );
    assert_eq!(T::try_from_parts(0, 0099).unwrap().saturating_mul(T::try_from_parts(0, 0053).unwrap()), T::ZERO);
    assert_eq!(
        T::try_from_parts(0, 9990).unwrap().saturating_mul(T::try_from_parts(0, 1000).unwrap()),
        T::try_from_parts(0, 0999).unwrap()
    );
    assert_eq!(T::MAX.saturating_mul(T::ZERO), T::ZERO);
    assert_eq!(T::MAX.saturating_mul(T::MAX), T::try_from_parts(0, 9998).unwrap());
}

#[test]
fn saturating_mul_10_9() {
    type T = UDf64<10, 9>;
    assert_eq!(
        T::try_from_parts(2, 718_280_000)
            .unwrap()
            .saturating_mul(T::try_from_parts(3, 141_500_000).unwrap()),
        T::try_from_parts(8, 539_476_620).unwrap()
    );
    assert_eq!(
        T::try_from_parts(2, 718_280_000)
            .unwrap()
            .saturating_mul(T::try_from_parts(3, 141_590_000).unwrap()),
        T::try_from_parts(8, 539_721_265).unwrap(),
    );
    assert_eq!(
        T::try_from_parts(72_850, 038_520_000)
            .unwrap()
            .saturating_mul(T::try_from_parts(92_820, 994_600_000).unwrap()),
        T::try_from_parts(6_762_013_032, 074_711_992).unwrap()
    );
    assert_eq!(
        T::try_from_parts(172_850, 038_520_000)
            .unwrap()
            .saturating_mul(T::try_from_parts(92_820, 994_600_000).unwrap()),
        T::MAX,
    );
    assert_eq!(
        T::try_from_parts(72_850, 038_520_000)
            .unwrap()
            .saturating_mul(T::try_from_parts(92_820, 994_660_000).unwrap()),
        T::try_from_parts(6_762_013_036, 445_714_303).unwrap(),
    );
    assert_eq!(
        T::try_from_parts(172_850, 038_520_000)
            .unwrap()
            .saturating_mul(T::try_from_parts(92_820, 994_660_000).unwrap()),
        T::MAX
    );
    assert_eq!(
        T::try_from_parts(0, 000_001_000)
            .unwrap()
            .saturating_mul(T::try_from_parts(0, 000_004_000).unwrap()),
        T::ZERO
    );
    assert_eq!(T::ONE.saturating_mul(T::ONE), T::ONE);
    assert_eq!(T::ONE.saturating_mul(T::ZERO), T::ZERO);
    assert_eq!(T::MAX.saturating_mul(T::ZERO), T::ZERO);
    assert_eq!(T::ONE.saturating_mul(T::MAX), T::MAX);
    assert_eq!(T::MAX.saturating_mul(T::MAX), T::MAX);
}

#[test]
fn approximate_pow_6_3() {
    type T = UDf64<6, 3>;
    assert_eq!(
        T::try_from_parts(42, 900).unwrap().approximate_pow(3),
        Ok(T::try_from_parts(78_953, 589).unwrap())
    );
    assert_eq!(T::try_from_parts(5, 100).unwrap().approximate_pow(3), Ok(T::try_from_parts(132, 651).unwrap()));
    assert_eq!(T::try_from_parts(76, 300).unwrap().approximate_pow(0), Ok(T::ONE));
    assert_eq!(T::try_from_parts(76, 300).unwrap().approximate_pow(1), Ok(T::try_from_parts(76, 300).unwrap()));
    assert_eq!(
        T::try_from_parts(76, 300).unwrap().approximate_pow(2),
        Ok(T::try_from_parts(5821, 690).unwrap())
    );
    assert_eq!(
        T::try_from_parts(76, 300).unwrap().approximate_pow(3),
        Ok(T::try_from_parts(444194, 947).unwrap())
    );
    {
        let x = T::try_from_parts(1, 234).unwrap().approximate_pow(56).unwrap();
        assert!(x >= T::try_from_parts(124145, 873).unwrap()); // 1.233^56
        assert!(x <= T::try_from_parts(135940, 738).unwrap()); // 1.235^56
    }
    {
        assert_eq!(T::try_from_parts(0, 500).unwrap().approximate_pow(10), Ok(T::ZERO));
    }
    assert_eq!(T::try_from_parts(1, 234).unwrap().approximate_pow(80), Err(ArithmeticError::ResultTooLarge));
    assert_eq!(T::try_from_parts(2, 0).unwrap().approximate_pow(19), Ok(T::try_from_parts(524_288, 0).unwrap()));
    assert_eq!(T::try_from_parts(2, 0).unwrap().approximate_pow(20), Err(ArithmeticError::ResultTooLarge));
    assert_eq!(T::ONE.approximate_pow(10), Ok(T::ONE));
    assert_eq!(T::ONE.approximate_pow(u32::MAX), Ok(T::ONE));
    assert_eq!(T::MAX.approximate_pow(0), Ok(T::ONE));
    assert_eq!(T::MAX.approximate_pow(1), Ok(T::MAX));
    assert_eq!(T::MAX.approximate_pow(2), Err(ArithmeticError::ResultTooLarge));
    assert_eq!(T::MAX.approximate_pow(u32::MAX), Err(ArithmeticError::ResultTooLarge));
}

#[test]
fn approximate_pow_4_0() {
    type T = UDf64<4, 0>;
    assert_eq!(T::try_from_parts(3, 0).unwrap().approximate_pow(3), Ok(T::try_from_parts(27, 0).unwrap()));
    assert_eq!(T::try_from_parts(3, 0).unwrap().approximate_pow(8), Ok(T::try_from_parts(6561, 0).unwrap()));
    assert_eq!(T::try_from_parts(12, 0).unwrap().approximate_pow(3), Ok(T::try_from_parts(1728, 0).unwrap()));
    assert_eq!(T::try_from_parts(12, 0).unwrap().approximate_pow(7), Err(ArithmeticError::ResultTooLarge));
    assert_eq!(T::ONE.approximate_pow(u32::MAX), Ok(T::ONE));
    assert_eq!(T::MAX.approximate_pow(0), Ok(T::ONE));
    assert_eq!(T::MAX.approximate_pow(1), Ok(T::MAX));
    assert_eq!(T::MAX.approximate_pow(2), Err(ArithmeticError::ResultTooLarge));
    assert_eq!(T::MAX.approximate_pow(u32::MAX), Err(ArithmeticError::ResultTooLarge));
}

#[test]
fn approximate_pow_10_9() {
    type T = UDf64<10, 9>;
    assert_eq!(T::try_from_parts(2, 0).unwrap().approximate_pow(0), Ok(T::try_from_parts(1, 0).unwrap()));
    assert_eq!(T::try_from_parts(2, 0).unwrap().approximate_pow(1), Ok(T::try_from_parts(2, 0).unwrap()));
    assert_eq!(T::try_from_parts(2, 0).unwrap().approximate_pow(2), Ok(T::try_from_parts(4, 0).unwrap()));
    assert_eq!(T::try_from_parts(2, 0).unwrap().approximate_pow(20), Ok(T::try_from_parts(1048576, 0).unwrap()));
    assert_eq!(
        T::try_from_parts(2, 0).unwrap().approximate_pow(31),
        Ok(T::try_from_parts(2_147_483_648, 0).unwrap())
    );
    assert_eq!(
        T::try_from_parts(2, 0).unwrap().approximate_pow(32),
        Ok(T::try_from_parts(4_294_967_296, 0).unwrap())
    );
    assert_eq!(T::try_from_parts(2, 0).unwrap().approximate_pow(u32::MAX), Err(ArithmeticError::ResultTooLarge));
    assert_eq!(
        T::try_from_parts(1, 100_000_000).unwrap().approximate_pow(9),
        Ok(T::try_from_parts(2, 357_947_691).unwrap())
    );
    assert_eq!(
        T::try_from_parts(12, 100_000_000).unwrap().approximate_pow(9),
        Ok(T::try_from_parts(5_559_917_313, 492_231_481).unwrap())
    );
    assert_eq!(
        T::try_from_parts(12, 100_000_000).unwrap().approximate_pow(10),
        Err(ArithmeticError::ResultTooLarge)
    );
    {
        let e = T::try_from_parts(2, 718_281_828).unwrap();
        assert_eq!(e.approximate_pow(0), Ok(T::ONE));
        assert_eq!(e.approximate_pow(1), Ok(e));
        let e10 = e.approximate_pow(10).unwrap();
        assert!(e10 >= T::try_from_parts(22_026, 465_676_579).unwrap());
        assert!(e10 <= T::try_from_parts(22_026, 465_838_640).unwrap());
        let e23 = e.approximate_pow(23).unwrap();
        assert!(e23 >= T::try_from_parts(9_744_803_325, 946_253_141).unwrap());
        assert!(e23 <= T::try_from_parts(9_744_803_490, 852_243_451).unwrap());
    }
    assert_eq!(T::ONE.approximate_pow(10), Ok(T::ONE));
    assert_eq!(T::ONE.approximate_pow(u32::MAX), Ok(T::ONE));
    assert_eq!(T::MAX.approximate_pow(0), Ok(T::ONE));
    assert_eq!(T::MAX.approximate_pow(1), Ok(T::MAX));
    assert_eq!(T::MAX.approximate_pow(2), Err(ArithmeticError::ResultTooLarge));
    assert_eq!(T::MAX.approximate_pow(u32::MAX), Err(ArithmeticError::ResultTooLarge));
}

#[test]
fn saturating_pow_6_3() {
    type T = UDf64<6, 3>;
    assert_eq!(T::try_from_parts(42, 900).unwrap().saturating_pow(3), T::try_from_parts(78_953, 589).unwrap());
    assert_eq!(T::try_from_parts(5, 100).unwrap().saturating_pow(3), T::try_from_parts(132, 651).unwrap());
    assert_eq!(T::try_from_parts(76, 300).unwrap().saturating_pow(0), T::ONE);
    assert_eq!(T::try_from_parts(76, 300).unwrap().saturating_pow(1), T::try_from_parts(76, 300).unwrap());
    assert_eq!(T::try_from_parts(76, 300).unwrap().saturating_pow(2), T::try_from_parts(5821, 690).unwrap());
    assert_eq!(T::try_from_parts(76, 300).unwrap().saturating_pow(3), T::try_from_parts(444194, 947).unwrap());
    {
        let x = T::try_from_parts(1, 234).unwrap().saturating_pow(56);
        assert!(x >= T::try_from_parts(124145, 873).unwrap()); // 1.233^56
        assert!(x <= T::try_from_parts(135940, 738).unwrap()); // 1.235^56
    }
    assert_eq!(T::try_from_parts(0, 500).unwrap().saturating_pow(10), T::ZERO);
    assert_eq!(T::try_from_parts(1, 234).unwrap().saturating_pow(80), T::MAX);
    assert_eq!(T::try_from_parts(2, 0).unwrap().saturating_pow(19), T::try_from_parts(524_288, 0).unwrap());
    assert_eq!(T::try_from_parts(2, 0).unwrap().saturating_pow(20), T::MAX);
    assert_eq!(T::ONE.saturating_pow(10), T::ONE);
    assert_eq!(T::ONE.saturating_pow(u32::MAX), T::ONE);
    assert_eq!(T::MAX.saturating_pow(0), T::ONE);
    assert_eq!(T::MAX.saturating_pow(1), T::MAX);
    assert_eq!(T::MAX.saturating_pow(2), T::MAX);
    assert_eq!(T::MAX.saturating_pow(u32::MAX), T::MAX);
}

#[test]
fn saturating_pow_4_0() {
    type T = UDf64<4, 0>;
    assert_eq!(T::try_from_parts(3, 0).unwrap().saturating_pow(3), T::try_from_parts(27, 0).unwrap());
    assert_eq!(T::try_from_parts(3, 0).unwrap().saturating_pow(8), T::try_from_parts(6561, 0).unwrap());
    assert_eq!(T::try_from_parts(12, 0).unwrap().saturating_pow(3), T::try_from_parts(1728, 0).unwrap());
    assert_eq!(T::try_from_parts(12, 0).unwrap().saturating_pow(7), T::MAX);
    assert_eq!(T::ONE.saturating_pow(u32::MAX), T::ONE);
    assert_eq!(T::MAX.saturating_pow(0), T::ONE);
    assert_eq!(T::MAX.saturating_pow(1), T::MAX);
    assert_eq!(T::MAX.saturating_pow(2), T::MAX);
    assert_eq!(T::MAX.saturating_pow(u32::MAX), T::MAX);
}

#[test]
fn saturating_pow_10_9() {
    type T = UDf64<10, 9>;
    assert_eq!(T::try_from_parts(2, 0).unwrap().saturating_pow(0), T::try_from_parts(1, 0).unwrap());
    assert_eq!(T::try_from_parts(2, 0).unwrap().saturating_pow(1), T::try_from_parts(2, 0).unwrap());
    assert_eq!(T::try_from_parts(2, 0).unwrap().saturating_pow(2), T::try_from_parts(4, 0).unwrap());
    assert_eq!(T::try_from_parts(2, 0).unwrap().saturating_pow(20), T::try_from_parts(1048576, 0).unwrap());
    assert_eq!(
        T::try_from_parts(2, 0).unwrap().saturating_pow(31),
        T::try_from_parts(2_147_483_648, 0).unwrap()
    );
    assert_eq!(
        T::try_from_parts(2, 0).unwrap().saturating_pow(32),
        T::try_from_parts(4_294_967_296, 0).unwrap()
    );
    assert_eq!(T::try_from_parts(2, 0).unwrap().saturating_pow(u32::MAX), T::MAX);
    assert_eq!(
        T::try_from_parts(1, 100_000_000).unwrap().saturating_pow(9),
        T::try_from_parts(2, 357_947_691).unwrap()
    );
    assert_eq!(
        T::try_from_parts(12, 100_000_000).unwrap().saturating_pow(9),
        T::try_from_parts(5_559_917_313, 492_231_481).unwrap()
    );
    assert_eq!(T::try_from_parts(12, 100_000_000).unwrap().saturating_pow(10), T::MAX);
    {
        let e = T::try_from_parts(2, 718_281_828).unwrap();
        assert_eq!(e.saturating_pow(0), T::ONE);
        assert_eq!(e.saturating_pow(1), e);
        let e10 = e.saturating_pow(10);
        assert!(e10 >= T::try_from_parts(22_026, 465_676_579).unwrap());
        assert!(e10 <= T::try_from_parts(22_026, 465_838_640).unwrap());
        let e23 = e.saturating_pow(23);
        assert!(e23 >= T::try_from_parts(9_744_803_325, 946_253_141).unwrap());
        assert!(e23 <= T::try_from_parts(9_744_803_490, 852_243_451).unwrap());
    }
    assert_eq!(T::ONE.saturating_pow(10), T::ONE);
    assert_eq!(T::ONE.saturating_pow(u32::MAX), T::ONE);
    assert_eq!(T::MAX.saturating_pow(0), T::ONE);
    assert_eq!(T::MAX.saturating_pow(1), T::MAX);
    assert_eq!(T::MAX.saturating_pow(2), T::MAX);
    assert_eq!(T::MAX.saturating_pow(u32::MAX), T::MAX);
}

#[test]
fn checked_div_6_3() {
    type T = UDf64<6, 3>;
    assert_eq!(
        T::try_from_parts(22, 500).unwrap().saturating_div(T::try_from_parts(9, 0).unwrap()),
        Ok(T::try_from_parts(2, 500).unwrap())
    );
    assert_eq!(
        T::try_from_parts(15, 0).unwrap().checked_div(T::try_from_parts(3, 0).unwrap()),
        Ok(T::try_from_parts(5, 0).unwrap())
    );
    assert_eq!(
        T::try_from_parts(24565, 548).unwrap().checked_div(T::try_from_parts(4, 360).unwrap()),
        Ok(T::try_from_parts(5634, 300).unwrap())
    );
    assert_eq!(
        T::try_from_parts(98454, 566).unwrap().checked_div(T::try_from_parts(0, 400).unwrap()),
        Ok(T::try_from_parts(246136, 415).unwrap())
    );
    assert_eq!(
        T::try_from_parts(128, 696).unwrap().checked_div(T::try_from_parts(0, 020).unwrap()),
        Ok(T::try_from_parts(6434, 800).unwrap())
    );
    assert_eq!(
        T::try_from_parts(346124, 378).unwrap().checked_div(T::try_from_parts(693, 950).unwrap()),
        Err(ArithmeticError::PrecisionLoss)
    );
    assert_eq!(T::ONE.checked_div(T::try_from_parts(3, 0).unwrap()), Err(ArithmeticError::PrecisionLoss));
    assert_eq!(
        T::try_from_parts(100_000, 0).unwrap().checked_div(T::try_from_parts(0, 100).unwrap()),
        Err(ArithmeticError::ResultTooLarge)
    );
    assert!(T::try_from_parts(213_389, 834)
        .unwrap()
        .checked_div(T::try_from_parts(0, 924).unwrap())
        .is_err());
    assert_eq!(T::ONE.checked_div(T::ZERO), Err(ArithmeticError::DivisionByZero));
    assert_eq!(
        T::try_from_parts(192_329, 608).unwrap().checked_div(T::ZERO),
        Err(ArithmeticError::DivisionByZero)
    );
    assert_eq!(T::ZERO.checked_div(T::ZERO), Err(ArithmeticError::DivisionByZero));
    assert_eq!(T::MAX.checked_div(T::ZERO), Err(ArithmeticError::DivisionByZero));
}

#[test]
fn checked_div_4_0() {
    type T = UDf64<4, 0>;
    assert_eq!(
        T::try_from_parts(8932, 0).unwrap().checked_div(T::try_from_parts(638, 0).unwrap()),
        Ok(T::try_from_parts(14, 0).unwrap())
    );
    assert_eq!(
        T::try_from_parts(4, 0).unwrap().checked_div(T::try_from_parts(8, 0).unwrap()),
        Err(ArithmeticError::PrecisionLoss)
    );
    assert_eq!(
        T::try_from_parts(4392, 0).unwrap().checked_div(T::try_from_parts(4, 0).unwrap()),
        Ok(T::try_from_parts(1098, 0).unwrap())
    );
    assert_eq!(T::MAX.checked_div(T::try_from_parts(9, 0).unwrap()), Ok(T::try_from_parts(1111, 0).unwrap()));
    assert_eq!(T::ONE.checked_div(T::ZERO), Err(ArithmeticError::DivisionByZero));
    assert_eq!(T::try_from_parts(2_329, 0).unwrap().checked_div(T::ZERO), Err(ArithmeticError::DivisionByZero));
    assert_eq!(T::ZERO.checked_div(T::ZERO), Err(ArithmeticError::DivisionByZero));
    assert_eq!(T::MAX.checked_div(T::ZERO), Err(ArithmeticError::DivisionByZero));
}

#[test]
fn checked_div_0_4() {
    type T = UDf64<0, 4>;
    assert_eq!(
        T::try_from_parts(0, 0518).unwrap().checked_div(T::try_from_parts(0, 1400).unwrap()),
        Ok(T::try_from_parts(0, 3700).unwrap())
    );
    assert_eq!(
        T::try_from_parts(0, 4000).unwrap().checked_div(T::try_from_parts(0, 2000).unwrap()),
        Err(ArithmeticError::ResultTooLarge)
    );
    assert_eq!(
        T::try_from_parts(0, 4392).unwrap().checked_div(T::try_from_parts(0, 8000).unwrap()),
        Ok(T::try_from_parts(0, 5490).unwrap())
    );
    assert_eq!(T::try_from_parts(0, 1).unwrap().checked_div(T::ZERO), Err(ArithmeticError::DivisionByZero));
    assert_eq!(T::try_from_parts(0, 9432).unwrap().checked_div(T::ZERO), Err(ArithmeticError::DivisionByZero));
    assert_eq!(T::ZERO.checked_div(T::ZERO), Err(ArithmeticError::DivisionByZero));
    assert_eq!(T::MAX.checked_div(T::ZERO), Err(ArithmeticError::DivisionByZero));
}

#[test]
fn checked_div_10_9() {
    type T = UDf64<10, 9>;
    assert_eq!(
        T::try_from_parts(8, 539_476_620)
            .unwrap()
            .checked_div(T::try_from_parts(3, 141_500_000).unwrap()),
        Ok(T::try_from_parts(2, 718_280_000).unwrap())
    );
    assert_eq!(
        T::try_from_parts(2_259_235, 697_444_200)
            .unwrap()
            .checked_div(T::try_from_parts(0, 035_600_00).unwrap()),
        Ok(T::try_from_parts(634_616_768, 945_000_000).unwrap())
    );
    assert_eq!(
        T::try_from_parts(3, 141_590_000)
            .unwrap()
            .checked_div(T::try_from_parts(2, 718_280_000).unwrap()),
        Err(ArithmeticError::PrecisionLoss),
    );
    assert_eq!(
        T::try_from_parts(6_762_013_032, 074_711_992)
            .unwrap()
            .checked_div(T::try_from_parts(92_820, 994_600_000).unwrap()),
        Ok(T::try_from_parts(72_850, 038_520_000).unwrap())
    );
    assert_eq!(
        T::try_from_parts(23_172_850, 038_500_000)
            .unwrap()
            .checked_div(T::try_from_parts(0, 000_400_000).unwrap()),
        Err(ArithmeticError::ResultTooLarge),
    );
    assert_eq!(
        T::try_from_parts(72_850, 038_520_000)
            .unwrap()
            .checked_div(T::try_from_parts(92_820, 994_660_000).unwrap()),
        Err(ArithmeticError::PrecisionLoss),
    );
    assert!(T::try_from_parts(172_850, 038_520_000)
        .unwrap()
        .checked_div(T::try_from_parts(92_820, 994_660_000).unwrap())
        .is_err());
    assert_eq!(
        T::try_from_parts(1, 829_382_774).unwrap().checked_div(T::MAX),
        Err(ArithmeticError::PrecisionLoss)
    );
    assert_eq!(T::ONE.checked_div(T::ONE), Ok(T::ONE));
    assert_eq!(T::ONE.checked_div(T::ZERO), Err(ArithmeticError::DivisionByZero));
    assert_eq!(T::MAX.checked_div(T::ZERO), Err(ArithmeticError::DivisionByZero));
    assert_eq!(T::ONE.checked_div(T::MAX), Err(ArithmeticError::PrecisionLoss));
    assert_eq!(T::MAX.checked_div(T::MAX), Ok(T::ONE));
}

#[test]
fn truncating_div_6_3() {
    type T = UDf64<6, 3>;
    assert_eq!(
        T::try_from_parts(22, 500).unwrap().saturating_div(T::try_from_parts(9, 0).unwrap()),
        Ok(T::try_from_parts(2, 500).unwrap())
    );
    assert_eq!(
        T::try_from_parts(15, 0).unwrap().truncating_div(T::try_from_parts(3, 0).unwrap()),
        Ok(T::try_from_parts(5, 0).unwrap())
    );
    assert_eq!(
        T::try_from_parts(24565, 548).unwrap().truncating_div(T::try_from_parts(4, 360).unwrap()),
        Ok(T::try_from_parts(5634, 300).unwrap())
    );
    assert_eq!(
        T::try_from_parts(98454, 566).unwrap().truncating_div(T::try_from_parts(0, 400).unwrap()),
        Ok(T::try_from_parts(246136, 415).unwrap())
    );
    assert_eq!(
        T::try_from_parts(128, 696).unwrap().truncating_div(T::try_from_parts(0, 020).unwrap()),
        Ok(T::try_from_parts(6434, 800).unwrap())
    );
    assert_eq!(
        T::try_from_parts(346124, 378).unwrap().truncating_div(T::try_from_parts(693, 950).unwrap()),
        Ok(T::try_from_parts(498, 774).unwrap())
    );
    assert_eq!(T::ONE.truncating_div(T::try_from_parts(3, 0).unwrap()), Ok(T::try_from_parts(0, 333).unwrap()));
    assert_eq!(
        T::try_from_parts(100_000, 0).unwrap().truncating_div(T::try_from_parts(0, 100).unwrap()),
        Err(ArithmeticError::ResultTooLarge)
    );
    assert_eq!(
        T::try_from_parts(213_389, 834).unwrap().truncating_div(T::try_from_parts(0, 924).unwrap()),
        Ok(T::try_from_parts(230_941, 378).unwrap())
    );
    assert_eq!(T::ONE.truncating_div(T::ZERO), Err(ArithmeticError::DivisionByZero));
    assert_eq!(
        T::try_from_parts(192_329, 608).unwrap().truncating_div(T::ZERO),
        Err(ArithmeticError::DivisionByZero)
    );
    assert_eq!(T::ZERO.truncating_div(T::ZERO), Err(ArithmeticError::DivisionByZero));
    assert_eq!(T::MAX.truncating_div(T::ZERO), Err(ArithmeticError::DivisionByZero));
}

#[test]
fn truncating_div_4_0() {
    type T = UDf64<4, 0>;
    assert_eq!(
        T::try_from_parts(8932, 0).unwrap().truncating_div(T::try_from_parts(638, 0).unwrap()),
        Ok(T::try_from_parts(14, 0).unwrap())
    );
    assert_eq!(T::try_from_parts(4, 0).unwrap().truncating_div(T::try_from_parts(8, 0).unwrap()), Ok(T::ZERO));
    assert_eq!(
        T::try_from_parts(4392, 0).unwrap().truncating_div(T::try_from_parts(4, 0).unwrap()),
        Ok(T::try_from_parts(1098, 0).unwrap())
    );
    assert_eq!(T::MAX.truncating_div(T::try_from_parts(9, 0).unwrap()), Ok(T::try_from_parts(1111, 0).unwrap()));
    assert_eq!(T::ONE.truncating_div(T::ZERO), Err(ArithmeticError::DivisionByZero));
    assert_eq!(
        T::try_from_parts(2_329, 0).unwrap().truncating_div(T::ZERO),
        Err(ArithmeticError::DivisionByZero)
    );
    assert_eq!(T::ZERO.truncating_div(T::ZERO), Err(ArithmeticError::DivisionByZero));
    assert_eq!(T::MAX.truncating_div(T::ZERO), Err(ArithmeticError::DivisionByZero));
}

#[test]
fn truncating_div_0_4() {
    type T = UDf64<0, 4>;
    assert_eq!(
        T::try_from_parts(0, 0518).unwrap().truncating_div(T::try_from_parts(0, 1400).unwrap()),
        Ok(T::try_from_parts(0, 3700).unwrap())
    );
    assert_eq!(
        T::try_from_parts(0, 4000).unwrap().truncating_div(T::try_from_parts(0, 2000).unwrap()),
        Err(ArithmeticError::ResultTooLarge)
    );
    assert_eq!(
        T::try_from_parts(0, 4392).unwrap().truncating_div(T::try_from_parts(0, 8000).unwrap()),
        Ok(T::try_from_parts(0, 5490).unwrap())
    );
    assert_eq!(T::try_from_parts(0, 1).unwrap().truncating_div(T::ZERO), Err(ArithmeticError::DivisionByZero));
    assert_eq!(
        T::try_from_parts(0, 9432).unwrap().truncating_div(T::ZERO),
        Err(ArithmeticError::DivisionByZero)
    );
    assert_eq!(T::ZERO.truncating_div(T::ZERO), Err(ArithmeticError::DivisionByZero));
    assert_eq!(T::MAX.truncating_div(T::ZERO), Err(ArithmeticError::DivisionByZero));
}

#[test]
fn truncating_div_10_9() {
    type T = UDf64<10, 9>;
    assert_eq!(
        T::try_from_parts(8, 539_476_620)
            .unwrap()
            .truncating_div(T::try_from_parts(3, 141_500_000).unwrap()),
        Ok(T::try_from_parts(2, 718_280_000).unwrap())
    );
    assert_eq!(
        T::try_from_parts(2_259_235, 697_444_200)
            .unwrap()
            .truncating_div(T::try_from_parts(0, 035_600_00).unwrap()),
        Ok(T::try_from_parts(634_616_768, 945_000_000).unwrap())
    );
    assert_eq!(
        T::try_from_parts(3, 141_590_000)
            .unwrap()
            .truncating_div(T::try_from_parts(2, 718_280_000).unwrap()),
        Ok(T::try_from_parts(1, 155_727_150).unwrap())
    );
    assert_eq!(
        T::try_from_parts(6_762_013_032, 074_711_992)
            .unwrap()
            .truncating_div(T::try_from_parts(92_820, 994_600_000).unwrap()),
        Ok(T::try_from_parts(72_850, 038_520_000).unwrap())
    );
    assert_eq!(
        T::try_from_parts(23_172_850, 038_500_000)
            .unwrap()
            .truncating_div(T::try_from_parts(0, 000_400_000).unwrap()),
        Err(ArithmeticError::ResultTooLarge),
    );
    assert_eq!(
        T::try_from_parts(72_850, 038_520_000)
            .unwrap()
            .truncating_div(T::try_from_parts(92_820, 994_660_000).unwrap()),
        Ok(T::try_from_parts(0, 784_844_407).unwrap())
    );
    assert_eq!(
        T::try_from_parts(172_850, 038_520_000)
            .unwrap()
            .truncating_div(T::try_from_parts(92_820, 994_660_000).unwrap()),
        Ok(T::try_from_parts(1, 862_186_880).unwrap())
    );
    assert_eq!(T::ONE.truncating_div(T::ONE), Ok(T::ONE));
    assert_eq!(T::ONE.truncating_div(T::ZERO), Err(ArithmeticError::DivisionByZero));
    assert_eq!(T::MAX.truncating_div(T::ZERO), Err(ArithmeticError::DivisionByZero));
    assert_eq!(T::ONE.truncating_div(T::MAX), Ok(T::ZERO));
    assert_eq!(T::MAX.truncating_div(T::MAX), Ok(T::ONE));
}

#[test]
fn saturating_div_6_3() {
    type T = UDf64<6, 3>;
    assert_eq!(
        T::try_from_parts(22, 500).unwrap().saturating_div(T::try_from_parts(9, 0).unwrap()),
        Ok(T::try_from_parts(2, 500).unwrap())
    );
    assert_eq!(
        T::try_from_parts(15, 0).unwrap().saturating_div(T::try_from_parts(3, 0).unwrap()),
        Ok(T::try_from_parts(5, 0).unwrap())
    );
    assert_eq!(
        T::try_from_parts(24565, 548).unwrap().saturating_div(T::try_from_parts(4, 360).unwrap()),
        Ok(T::try_from_parts(5634, 300).unwrap())
    );
    assert_eq!(
        T::try_from_parts(98454, 566).unwrap().saturating_div(T::try_from_parts(0, 400).unwrap()),
        Ok(T::try_from_parts(246136, 415).unwrap())
    );
    assert_eq!(
        T::try_from_parts(128, 696).unwrap().saturating_div(T::try_from_parts(0, 020).unwrap()),
        Ok(T::try_from_parts(6434, 800).unwrap())
    );
    assert_eq!(
        T::try_from_parts(346124, 378).unwrap().saturating_div(T::try_from_parts(693, 950).unwrap()),
        Ok(T::try_from_parts(498, 774).unwrap())
    );
    assert_eq!(T::ONE.saturating_div(T::try_from_parts(3, 0).unwrap()), Ok(T::try_from_parts(0, 333).unwrap()));
    assert_eq!(
        T::try_from_parts(100_000, 0).unwrap().saturating_div(T::try_from_parts(0, 100).unwrap()),
        Ok(T::MAX)
    );
    assert_eq!(
        T::try_from_parts(213_389, 834).unwrap().saturating_div(T::try_from_parts(0, 924).unwrap()),
        Ok(T::try_from_parts(230_941, 378).unwrap())
    );
    assert_eq!(T::ONE.saturating_div(T::ZERO), Err(ArithmeticError::DivisionByZero));
    assert_eq!(
        T::try_from_parts(192_329, 608).unwrap().saturating_div(T::ZERO),
        Err(ArithmeticError::DivisionByZero)
    );
    assert_eq!(T::ZERO.saturating_div(T::ZERO), Err(ArithmeticError::DivisionByZero));
    assert_eq!(T::MAX.saturating_div(T::ZERO), Err(ArithmeticError::DivisionByZero));
}

#[test]
fn saturating_div_4_0() {
    type T = UDf64<4, 0>;
    assert_eq!(
        T::try_from_parts(8932, 0).unwrap().saturating_div(T::try_from_parts(638, 0).unwrap()),
        Ok(T::try_from_parts(14, 0).unwrap())
    );
    assert_eq!(T::try_from_parts(4, 0).unwrap().saturating_div(T::try_from_parts(8, 0).unwrap()), Ok(T::ZERO));
    assert_eq!(
        T::try_from_parts(4392, 0).unwrap().saturating_div(T::try_from_parts(4, 0).unwrap()),
        Ok(T::try_from_parts(1098, 0).unwrap())
    );
    assert_eq!(T::MAX.saturating_div(T::try_from_parts(9, 0).unwrap()), Ok(T::try_from_parts(1111, 0).unwrap()));
    assert_eq!(T::ONE.saturating_div(T::ZERO), Err(ArithmeticError::DivisionByZero));
    assert_eq!(
        T::try_from_parts(2_329, 0).unwrap().saturating_div(T::ZERO),
        Err(ArithmeticError::DivisionByZero)
    );
    assert_eq!(T::ZERO.saturating_div(T::ZERO), Err(ArithmeticError::DivisionByZero));
    assert_eq!(T::MAX.saturating_div(T::ZERO), Err(ArithmeticError::DivisionByZero));
}

#[test]
fn saturating_div_0_4() {
    type T = UDf64<0, 4>;
    assert_eq!(
        T::try_from_parts(0, 0518).unwrap().saturating_div(T::try_from_parts(0, 1400).unwrap()),
        Ok(T::try_from_parts(0, 3700).unwrap())
    );
    assert_eq!(
        T::try_from_parts(0, 4000).unwrap().saturating_div(T::try_from_parts(0, 2000).unwrap()),
        Ok(T::MAX)
    );
    assert_eq!(
        T::try_from_parts(0, 4392).unwrap().saturating_div(T::try_from_parts(0, 8000).unwrap()),
        Ok(T::try_from_parts(0, 5490).unwrap())
    );
    assert_eq!(T::try_from_parts(0, 1).unwrap().saturating_div(T::ZERO), Err(ArithmeticError::DivisionByZero));
    assert_eq!(
        T::try_from_parts(0, 9432).unwrap().saturating_div(T::ZERO),
        Err(ArithmeticError::DivisionByZero)
    );
    assert_eq!(T::ZERO.saturating_div(T::ZERO), Err(ArithmeticError::DivisionByZero));
    assert_eq!(T::MAX.saturating_div(T::ZERO), Err(ArithmeticError::DivisionByZero));
}

#[test]
fn saturating_div_10_9() {
    type T = UDf64<10, 9>;
    assert_eq!(
        T::try_from_parts(8, 539_476_620)
            .unwrap()
            .saturating_div(T::try_from_parts(3, 141_500_000).unwrap()),
        Ok(T::try_from_parts(2, 718_280_000).unwrap())
    );
    assert_eq!(
        T::try_from_parts(2_259_235, 697_444_200)
            .unwrap()
            .saturating_div(T::try_from_parts(0, 035_600_00).unwrap()),
        Ok(T::try_from_parts(634_616_768, 945_000_000).unwrap())
    );
    assert_eq!(
        T::try_from_parts(3, 141_590_000)
            .unwrap()
            .saturating_div(T::try_from_parts(2, 718_280_000).unwrap()),
        Ok(T::try_from_parts(1, 155_727_150).unwrap())
    );
    assert_eq!(
        T::try_from_parts(6_762_013_032, 074_711_992)
            .unwrap()
            .saturating_div(T::try_from_parts(92_820, 994_600_000).unwrap()),
        Ok(T::try_from_parts(72_850, 038_520_000).unwrap())
    );
    assert_eq!(
        T::try_from_parts(23_172_850, 038_500_000)
            .unwrap()
            .saturating_div(T::try_from_parts(0, 000_400_000).unwrap()),
        Ok(T::MAX)
    );
    assert_eq!(
        T::try_from_parts(72_850, 038_520_000)
            .unwrap()
            .saturating_div(T::try_from_parts(92_820, 994_660_000).unwrap()),
        Ok(T::try_from_parts(0, 784_844_407).unwrap())
    );
    assert_eq!(
        T::try_from_parts(172_850, 038_520_000)
            .unwrap()
            .saturating_div(T::try_from_parts(92_820, 994_660_000).unwrap()),
        Ok(T::try_from_parts(1, 862_186_880).unwrap())
    );
    assert_eq!(T::ONE.saturating_div(T::ONE), Ok(T::ONE));
    assert_eq!(T::ONE.saturating_div(T::ZERO), Err(ArithmeticError::DivisionByZero));
    assert_eq!(T::MAX.saturating_div(T::ZERO), Err(ArithmeticError::DivisionByZero));
    assert_eq!(T::ONE.saturating_div(T::MAX), Ok(T::ZERO));
    assert_eq!(T::MAX.saturating_div(T::MAX), Ok(T::ONE));
}

#[test]
fn fmt_6_3() {
    type T = UDf64<6, 3>;
    assert_eq!(T::try_from_parts(12, 345).unwrap().display().to_string(), "12.345");
    assert_eq!(T::try_from_parts(12345, 600).unwrap().display().to_string(), "12345.600");
    assert_eq!(T::ZERO.display().to_string(), "0.000");
    assert_eq!(T::ONE.display().to_string(), "1.000");
    assert_eq!(T::MAX.display().to_string(), "999999.999");
}

#[test]
fn fmt_4_0() {
    type T = UDf64<4, 0>;
    assert_eq!(T::try_from_parts(12, 0).unwrap().display().to_string(), "12.0");
    assert_eq!(T::try_from_parts(1234, 0).unwrap().display().to_string(), "1234.0");
    assert_eq!(T::ZERO.display().to_string(), "0.0");
    assert_eq!(T::ONE.display().to_string(), "1.0");
    assert_eq!(T::MAX.display().to_string(), "9999.0");
}

#[test]
fn fmt_0_4() {
    type T = UDf64<0, 4>;
    assert_eq!(T::try_from_parts(0, 1200).unwrap().display().to_string(), "0.1200");
    assert_eq!(T::try_from_parts(0, 12).unwrap().display().to_string(), "0.0012");
    assert_eq!(T::try_from_parts(0, 1234).unwrap().display().to_string(), "0.1234");
    assert_eq!(T::ZERO.display().to_string(), "0.0000");
    assert_eq!(T::MAX.display().to_string(), "0.9999");
}

#[test]
fn fmt_10_9() {
    type T = UDf64<10, 9>;
    assert_eq!(T::try_from_parts(12, 345678901).unwrap().display().to_string(), "12.345678901");
    assert_eq!(T::try_from_parts(12345, 000_600_000).unwrap().display().to_string(), "12345.000600000");
    assert_eq!(T::try_from_parts(2, 718_281_828).unwrap().display().to_string(), "2.718281828");
    assert_eq!(
        T::try_from_parts(1_234_567_890, 123_456_789).unwrap().display().to_string(),
        "1234567890.123456789"
    );
    assert_eq!(T::try_from_parts(12_398_742, 300_239_991).unwrap().display().to_string(), "12398742.300239991");
    assert_eq!(T::ZERO.display().to_string(), "0.000000000");
    assert_eq!(T::ONE.display().to_string(), "1.000000000");
    assert_eq!(T::MAX.display().to_string(), "9999999999.999999999");
}

#[test]
fn fmt_19_0() {
    type T = UDf64<19, 0>;
    assert_eq!(T::try_from_parts(12345678901, 0).unwrap().display().to_string(), "12345678901.0");
    assert_eq!(T::try_from_parts(12345000600000, 0).unwrap().display().to_string(), "12345000600000.0");
    assert_eq!(T::try_from_parts(2718281828, 0).unwrap().display().to_string(), "2718281828.0");
    assert_eq!(
        T::try_from_parts(1234567890123456789, 0).unwrap().display().to_string(),
        "1234567890123456789.0"
    );
    assert_eq!(T::try_from_parts(12398742300239991, 0).unwrap().display().to_string(), "12398742300239991.0");
    assert_eq!(T::ZERO.display().to_string(), "0.0");
    assert_eq!(T::ONE.display().to_string(), "1.0");
    assert_eq!(T::MAX.display().to_string(), "9999999999999999999.0");
}

#[test]
fn fmt_0_19() {
    type T = UDf64<0, 19>;
    assert_eq!(
        T::try_from_parts(0, 1234567890100000000).unwrap().display().to_string(),
        "0.1234567890100000000"
    );
    assert_eq!(
        T::try_from_parts(0, 0000012345000600000).unwrap().display().to_string(),
        "0.0000012345000600000"
    );
    assert_eq!(
        T::try_from_parts(0, 0000000002718281828).unwrap().display().to_string(),
        "0.0000000002718281828"
    );
    assert_eq!(
        T::try_from_parts(0, 1234567890123456789).unwrap().display().to_string(),
        "0.1234567890123456789"
    );
    assert_eq!(
        T::try_from_parts(0, 0012398742300239991).unwrap().display().to_string(),
        "0.0012398742300239991"
    );
    assert_eq!(T::ZERO.display().to_string(), "0.0000000000000000000");
    assert_eq!(T::MAX.display().to_string(), "0.9999999999999999999");
}
