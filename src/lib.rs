//! Primitive integer division with rounding kind and cumulative error.
//!
//! As we all know, integer division can lead to precision loss and errors.
//! What is less obvious is that in some scenarios, the dividend is split
//! into several parts, turning a single division into multiple divisions,
//! which can sometimes result in even greater errors.
//!
//! For example, 60 / 3 = 20, and this division itself is error-free. However,
//! if we split the dividend 60 into three parts of 20 and perform the
//! division three times: 20 / 3 = 6.67, rounding it to 7 (using the Rounded
//! method as an example; other methods are similar). Adding up the three 7
//! gives us 21, which is 1 more than the original 20.
//!
//! For such consecutive divisions, if we can record the error caused by
//! rounding in each division and apply it to the next division, we can
//! reduce or even avoid the final error.
//!
//! Let's use `cum_error` to denote the cumulative error.
//!
//! - The initial value is cum_error = 0;
//! - (20 + 0) / 3 = 7, cum_error = -1;
//! - (20 - 1) / 3 = 6, cum_error = 1;
//! - (20 + 1) / 3 = 7, cum_error = 0.
//!
//! The final result is 7 + 6 + 7 = 20, which is equal to the result of
//! the single division.
//!
//! ```
//! use int_div_cum_error::*;
//!
//! let mut cum_error = 0;
//!
//! let q = checked_divide_with_cum_error(20, 3, Rounding::Round, &mut cum_error).unwrap();
//! assert_eq!(q, 7);
//! assert_eq!(cum_error, -1);
//!
//! let q = checked_divide_with_cum_error(20, 3, Rounding::Round, &mut cum_error).unwrap();
//! assert_eq!(q, 6);
//! assert_eq!(cum_error, 1);
//!
//! let q = checked_divide_with_cum_error(20, 3, Rounding::Round, &mut cum_error).unwrap();
//! assert_eq!(q, 7);
//! assert_eq!(cum_error, 0);
//! ```
//!
//! This library implements this functionality.

use num_traits::{int::PrimInt, sign::Signed, identities::{ConstOne, ConstZero}, CheckedAdd};

/// Rounding kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Rounding {
    /// towards the nearest integer
    #[default]
    Round,
    /// towards negative infinity
    Floor,
    /// towards positive infinity
    Ceiling,
    /// towards zero
    TowardsZero,
    /// away from zero
    AwayFromZero,
}

use std::ops::{AddAssign, SubAssign};

/// Alias of trait set for primitive signed integers.
pub trait PrimSignedInt: PrimInt + Signed + ConstOne + ConstZero + AddAssign + SubAssign {
    fn unsigned_abs(self) -> impl PrimInt;
}

impl PrimSignedInt for i8 {
    fn unsigned_abs(self) -> impl PrimInt {
        i8::unsigned_abs(self)
    }
}
impl PrimSignedInt for i16 {
    fn unsigned_abs(self) -> impl PrimInt {
        i16::unsigned_abs(self)
    }
}
impl PrimSignedInt for i32 {
    fn unsigned_abs(self) -> impl PrimInt {
        i32::unsigned_abs(self)
    }
}
impl PrimSignedInt for i64 {
    fn unsigned_abs(self) -> impl PrimInt {
        i64::unsigned_abs(self)
    }
}
impl PrimSignedInt for i128 {
    fn unsigned_abs(self) -> impl PrimInt {
        i128::unsigned_abs(self)
    }
}

// return: abs(a) >= abs(b)
fn cmp_abs_ge<I>(a: I, b: I) -> bool
where I: PrimSignedInt
{
    a.unsigned_abs() >= b.unsigned_abs()
}

// return: abs(a) * 2 >= abs(b)
fn cmp_abs_half_ge<I>(a: I, b: I) -> bool
where I: PrimSignedInt
{
    let a = a.unsigned_abs();
    match a.checked_add(&a) {
        Some(a2) => a2 >= b.unsigned_abs(),
        None => true,
    }
}

// return: abs(a + b) < abs(b)
fn add_cmp_abs_lt<I>(a: I, b: I) -> bool
where I: PrimSignedInt
{
    match a.checked_add(&b) {
        None => false,
        Some(n) => !cmp_abs_ge(n, b),
    }
}

// return: abs(a - b) < abs(b)
fn sub_cmp_abs_lt<I>(a: I, b: I) -> bool
where I: PrimSignedInt
{
    match a.checked_sub(&b) {
        None => false,
        Some(n) => !cmp_abs_ge(n, b),
    }
}

// return: a and b have same sign
fn same_sign<I>(a: I, b: I) -> bool
where I: PrimSignedInt
{
    (a ^ b).is_positive()
}


/// Checked division with rounding kind specified.
///
/// Return `None` if divided by 0 or overflow occurrs.
///
/// `I` is some primitive signed integer type,
/// such as `i8`, `i16`, `132`, `i64`, or `i128`.
///
/// This is a common implementation for integer division and rouding.
/// It has no relation to the cumulative error in this crate.
pub fn checked_divide_with_rounding<I>(
    left: I,
    right: I,
    rounding: Rounding,
) -> Option<I>
where I: PrimSignedInt
{
    let Some(q) = left.checked_div(&right) else {
        return None;
    };

    let remain = left % right;
    if remain.is_zero() {
        return Some(q);
    }

    Some(match rounding {
        Rounding::Floor => {
            if same_sign(left, right) {
                q
            } else {
                q - I::ONE
            }
        }
        Rounding::Ceiling => {
            if same_sign(left, right) {
                q + I::ONE
            } else {
                q
            }
        }
        Rounding::Round => {
            if cmp_abs_half_ge(remain, right) {
                if same_sign(left, right) {
                    q + I::ONE
                } else {
                    q - I::ONE
                }
            } else {
                q
            }
        }
        Rounding::TowardsZero => {
            q
        }
        Rounding::AwayFromZero => {
            if same_sign(left, right) {
                q + I::ONE
            } else {
                q - I::ONE
            }
        }
    })
}

/// Checked division with rounding kind and cumulative error specified.
///
/// Return `None` if divided by 0 or overflow occurrs.
///
/// `I` is some primitive signed integer type,
/// such as `i8`, `i16`, `132`, `i64`, or `i128`.
///
/// See [the module-level documentation](index.html) for more information
/// of `cum_error`. If you do not need `cum_error`, then use
/// [`checked_divide_with_rounding`] which might be a little faster.
pub fn checked_divide_with_cum_error<I>(
    left: I,
    right: I,
    rounding: Rounding,
    cum_error: &mut I,
) -> Option<I>
where I: PrimSignedInt
{
    let Some(mut q) = left.checked_div(&right) else {
        return None;
    };

    let remain = left % right;
    if remain.is_zero() {
        return Some(q);
    }

    let Some(tmpsum) = cum_error.checked_add(&remain) else {
        if same_sign(left, right) {
            *cum_error += remain - right;
            return Some(q + I::ONE);
        } else {
            *cum_error += remain + right;
            return Some(q - I::ONE);
        }
    };
    *cum_error = tmpsum;

    match rounding {
        Rounding::Floor => {
            if same_sign(left, right) {
                if cmp_abs_ge(*cum_error, right) {
                    *cum_error -= right;
                    q += I::ONE;
                }
            } else {
                if add_cmp_abs_lt(*cum_error, right) {
                    *cum_error += right;
                    q -= I::ONE;
                }
            }
        }
        Rounding::Ceiling => {
            if same_sign(left, right) {
                if sub_cmp_abs_lt(*cum_error, right) {
                    *cum_error -= right;
                    q += I::ONE;
                }
            } else {
                if cmp_abs_ge(*cum_error, right) {
                    *cum_error += right;
                    q -= I::ONE;
                }
            }
        }
        Rounding::Round => {
            if cmp_abs_half_ge(*cum_error, right) {
                if same_sign(left, right) {
                    *cum_error -= right;
                    q += I::ONE;
                } else {
                    *cum_error += right;
                    q -= I::ONE;
                }
            }
        }
        Rounding::TowardsZero => {
            if cmp_abs_ge(*cum_error, right) {
                if same_sign(left, right) {
                    *cum_error -= right;
                    q += I::ONE;
                } else {
                    *cum_error += right;
                    q -= I::ONE;
                }
            }
        }
        Rounding::AwayFromZero => {
            if same_sign(left, right) {
                if sub_cmp_abs_lt(*cum_error, right) {
                    *cum_error -= right;
                    q += I::ONE;
                }
            } else {
                if add_cmp_abs_lt(*cum_error, right) {
                    *cum_error += right;
                    q -= I::ONE;
                }
            }
        }
    }
    Some(q)
}

/// Wrapper of [`checked_divide_with_rounding`] and [`checked_divide_with_cum_error`] functions.
///
/// It calls `checked_divide_with_rounding` if `cum_error` is `None`, or calls
/// `checked_divide_with_cum_error` if not None.
pub fn checked_divide<I>(
    left: I,
    right: I,
    rounding: Rounding,
    cum_error: Option<&mut I>,
) -> Option<I>
where I: PrimSignedInt
{
    match cum_error {
        Some(cum_error) => checked_divide_with_cum_error(left, right, rounding, cum_error),
        None => checked_divide_with_rounding(left, right, rounding),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_rounding_fib(a: i32, b: i32, rounding: Rounding) {
        let mut cum_error = 0_i32;
        let mut i0 = a;
        let mut i1 = a;
        let mut isum = 0_i32;
        let mut ret = 0_i32;
        while isum.abs() < 1_000_000_000 {
            let ix = i0 + i1;

            ret += checked_divide_with_cum_error(ix, b, rounding, &mut cum_error).unwrap();
            i0 = i1;
            i1 = ix;
            isum += ix;

            let q = checked_divide_with_rounding(isum, b, rounding).unwrap();
            let r = isum - q * b;
            assert_eq!(q, ret);
            assert_eq!(r, cum_error);
        }
    }

    fn do_test(a: i32, b: i32) {
        test_rounding_fib(a, b, Rounding::Floor);
        test_rounding_fib(a, b, Rounding::Ceiling);
        test_rounding_fib(a, b, Rounding::Round);
        test_rounding_fib(a, b, Rounding::AwayFromZero);
        test_rounding_fib(a, b, Rounding::TowardsZero);
    }

    fn test(b: i32) {
        do_test(1, b);
        do_test(-1, b);
        do_test(1, -b);
        do_test(-1, -b);
    }

    #[test]
    fn many_test() {
        for b in 1..100 {
            test(b*3+6);
            test(b*13+7);
            test(b*113+8);
            test(b*1113+9);
            test(b*11113+9);
            test(b*111113+9);
            test(b*1111113+9);
        }
        println!("done");
    }

    fn do_test_overflow(a: i32, b: i32) {
        let mut cum_error = 0_i32;
        checked_divide_with_cum_error(a, b, Rounding::Floor, &mut cum_error).unwrap();
        checked_divide_with_cum_error(a, b, Rounding::Floor, &mut cum_error).unwrap();
        checked_divide_with_cum_error(a, b, Rounding::Floor, &mut cum_error).unwrap();
        checked_divide_with_cum_error(a, b, Rounding::Floor, &mut cum_error).unwrap();
        let mut cum_error = 0_i32;
        checked_divide_with_cum_error(a, b, Rounding::Ceiling, &mut cum_error).unwrap();
        checked_divide_with_cum_error(a, b, Rounding::Ceiling, &mut cum_error).unwrap();
        checked_divide_with_cum_error(a, b, Rounding::Ceiling, &mut cum_error).unwrap();
        checked_divide_with_cum_error(a, b, Rounding::Ceiling, &mut cum_error).unwrap();
        let mut cum_error = 0_i32;
        checked_divide_with_cum_error(a, b, Rounding::Round, &mut cum_error).unwrap();
        checked_divide_with_cum_error(a, b, Rounding::Round, &mut cum_error).unwrap();
        checked_divide_with_cum_error(a, b, Rounding::Round, &mut cum_error).unwrap();
        checked_divide_with_cum_error(a, b, Rounding::Round, &mut cum_error).unwrap();
        let mut cum_error = 0_i32;
        checked_divide_with_cum_error(a, b, Rounding::TowardsZero, &mut cum_error).unwrap();
        checked_divide_with_cum_error(a, b, Rounding::TowardsZero, &mut cum_error).unwrap();
        checked_divide_with_cum_error(a, b, Rounding::TowardsZero, &mut cum_error).unwrap();
        checked_divide_with_cum_error(a, b, Rounding::TowardsZero, &mut cum_error).unwrap();
        let mut cum_error = 0_i32;
        checked_divide_with_cum_error(a, b, Rounding::AwayFromZero, &mut cum_error).unwrap();
        checked_divide_with_cum_error(a, b, Rounding::AwayFromZero, &mut cum_error).unwrap();
        checked_divide_with_cum_error(a, b, Rounding::AwayFromZero, &mut cum_error).unwrap();
        checked_divide_with_cum_error(a, b, Rounding::AwayFromZero, &mut cum_error).unwrap();
    }

    #[test]
    fn test_overflow() {
        do_test_overflow(i32::MAX - 100, i32::MAX);
        do_test_overflow(i32::MIN + 100, i32::MIN);
        do_test_overflow(i32::MAX - 100, i32::MIN);
        do_test_overflow(i32::MIN + 100, i32::MAX);

        do_test_overflow(i32::MAX / 2, i32::MAX);
        do_test_overflow(i32::MIN / 2, i32::MIN);
        do_test_overflow(i32::MAX / 2, i32::MIN);
        do_test_overflow(i32::MIN / 2, i32::MAX);

        do_test_overflow(i32::MAX / 3, i32::MAX);
        do_test_overflow(i32::MIN / 3, i32::MIN);
        do_test_overflow(i32::MAX / 3, i32::MIN);
        do_test_overflow(i32::MIN / 3, i32::MAX);

        do_test_overflow(i32::MAX / 3 * 2, i32::MAX);
        do_test_overflow(i32::MIN / 3 * 2, i32::MIN);
        do_test_overflow(i32::MAX / 3 * 2, i32::MIN);
        do_test_overflow(i32::MIN / 3 * 2, i32::MAX);
    }
}
