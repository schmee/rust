// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Operations and constants for signed 16-bits integers (`i16` type)

use default::Default;
use intrinsics;
use num::{Bitwise, Bounded, Zero, One, Signed, Num, Primitive, Int};
use num::{CheckedDiv, CheckedAdd, CheckedSub, CheckedMul};
use option::{Option, Some, None};

#[cfg(not(test))]
use cmp::{Eq, Ord, TotalEq, TotalOrd, Less, Greater, Equal, Ordering};
#[cfg(not(test))]
use ops::{Add, Sub, Mul, Div, Rem, Neg, BitOr, BitAnd, BitXor};
#[cfg(not(test))]
use ops::{Shl, Shr, Not};

int_module!(i16, 16)

impl Bitwise for i16 {
    /// Returns the number of ones in the binary representation of the number.
    #[inline]
    fn count_ones(&self) -> i16 { unsafe { intrinsics::ctpop16(*self as u16) as i16 } }

    /// Returns the number of leading zeros in the in the binary representation
    /// of the number.
    #[inline]
    fn leading_zeros(&self) -> i16 { unsafe { intrinsics::ctlz16(*self as u16) as i16 } }

    /// Returns the number of trailing zeros in the in the binary representation
    /// of the number.
    #[inline]
    fn trailing_zeros(&self) -> i16 { unsafe { intrinsics::cttz16(*self as u16) as i16 } }
}

impl CheckedAdd for i16 {
    #[inline]
    fn checked_add(&self, v: &i16) -> Option<i16> {
        unsafe {
            let (x, y) = intrinsics::i16_add_with_overflow(*self, *v);
            if y { None } else { Some(x) }
        }
    }
}

impl CheckedSub for i16 {
    #[inline]
    fn checked_sub(&self, v: &i16) -> Option<i16> {
        unsafe {
            let (x, y) = intrinsics::i16_sub_with_overflow(*self, *v);
            if y { None } else { Some(x) }
        }
    }
}

impl CheckedMul for i16 {
    #[inline]
    fn checked_mul(&self, v: &i16) -> Option<i16> {
        unsafe {
            let (x, y) = intrinsics::i16_mul_with_overflow(*self, *v);
            if y { None } else { Some(x) }
        }
    }
}
