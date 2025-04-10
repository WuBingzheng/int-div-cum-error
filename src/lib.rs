//! Integer division with rounding kind and cumulative error.
//!
//!
//!

/// Rounding kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Rounding {
    /// towards the nearest integer
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

/// Checked division with rounding kind specified.
///
/// Return `None` if divided by 0 or overflow occurrs, or `Some(int)` where `int`
/// is the type of `left` and `right`, which is some primitive signed integer type,
/// such as `i8`, `i16`, `132`, `i64`, or `i128`.
#[macro_export]
macro_rules! checked_divide {
    ($left:expr, $right:expr, $rounding:expr) => {
        'a: {
            let Some(q) = $left.checked_div($right) else {
                break 'a None;
            };

            let remain = $left % $right;
            if remain == 0 {
                break 'a Some(q);
            }

            Some(match $rounding {
                Rounding::Floor => {
                    if ($left ^ $right) > 0 {
                        q
                    } else {
                        q - 1
                    }
                }
                Rounding::Ceiling => {
                    if ($left ^ $right) > 0 {
                        q + 1
                    } else {
                        q
                    }
                }
                Rounding::Round => {
                    if remain.unsigned_abs() >= $right.unsigned_abs() / 2 {
                        if ($left ^ $right) > 0 {
                            q + 1
                        } else {
                            q - 1
                        }
                    } else {
                        q
                    }
                }
                Rounding::TowardsZero => {
                    q
                }
                Rounding::AwayFromZero => {
                    if ($left ^ $right) > 0 {
                        q + 1
                    } else {
                        q - 1
                    }
                }
            })
        }
    }
}

pub fn checked_divide_i8(lhs: i8, rhs: i8, rounding: Rounding) -> Option<i8> {
    checked_divide!(lhs, rhs, rounding)
}
pub fn checked_divide_i16(lhs: i16, rhs: i16, rounding: Rounding) -> Option<i16> {
    checked_divide!(lhs, rhs, rounding)
}
pub fn checked_divide_i32(lhs: i32, rhs: i32, rounding: Rounding) -> Option<i32> {
    checked_divide!(lhs, rhs, rounding)
}
pub fn checked_divide_i64(lhs: i64, rhs: i64, rounding: Rounding) -> Option<i64> {
    checked_divide!(lhs, rhs, rounding)
}
pub fn checked_divide_i128(lhs: i128, rhs: i128, rounding: Rounding) -> Option<i128> {
    checked_divide!(lhs, rhs, rounding)
}

// return: abs(a + b) < abs(b)
#[allow(unused_macros)]
macro_rules! add_cmp_abs {
    ($a:expr, $b:expr) => {
        match $a.checked_add($b) {
            None => false,
            Some(n) => n.unsigned_abs() < $b.unsigned_abs(),
        }
    }
}

// return: abs(a - b) < abs(b)
#[allow(unused_macros)]
macro_rules! sub_cmp_abs {
    ($a:expr, $b:expr) => {
        match $a.checked_sub($b) {
            None => false,
            Some(n) => n.unsigned_abs() < $b.unsigned_abs(),
        }
    }
}

/// Checked division with rounding kind and cumulative error specified.
///
/// Return `None` if divided by 0 or overflow occurrs, or `Some(int)` where `int`
/// is the type of `left` and `right`, which is some primitive signed integer type,
/// such as `i8`, `i16`, `132`, `i64`, or `i128`.
///
/// The `$cum_error` is cumulative error, which is in `&mut int` type.
/// See [the module-level documentation](index.html) for more information.
#[macro_export]
macro_rules! checked_divide_with_cum_error {
    ($left:expr, $right:expr, $rounding:expr, $cum_error:expr) => {
        'a: {
            let Some(mut q) = $left.checked_div($right) else {
                break 'a None;
            };

            let remain = $left % $right;
            if remain == 0 {
                break 'a Some(q);
            }

            let Some(tmpsum) = $cum_error.checked_add(remain) else {
                if ($left ^ $right) > 0 {
                    *$cum_error += remain - $right;
                    break 'a Some(q + 1);
                } else {
                    *$cum_error += remain + $right;
                    break 'a Some(q - 1);
                }
            };
            *$cum_error = tmpsum;

            match $rounding {
                Rounding::Floor => {
                    if ($left ^ $right) > 0 {
                        if $cum_error.unsigned_abs() >= $right.unsigned_abs() {
                            *$cum_error -= $right;
                            q += 1;
                        }
                    } else {
                        if add_cmp_abs!(*$cum_error, $right) {
                            *$cum_error += $right;
                            q -= 1;
                        }
                    }
                }
                Rounding::Ceiling => {
                    if ($left ^ $right) > 0 {
                        if sub_cmp_abs!(*$cum_error, $right) {
                            *$cum_error -= $right;
                            q += 1;
                        }
                    } else {
                        if $cum_error.unsigned_abs() >= $right.unsigned_abs() {
                            *$cum_error += $right;
                            q -= 1;
                        }
                    }
                }
                Rounding::Round => {
                    if $cum_error.unsigned_abs() >= $right.unsigned_abs() / 2 {
                        if ($left ^ $right) > 0 {
                            *$cum_error -= $right;
                            q += 1;
                        } else {
                            *$cum_error += $right;
                            q -= 1;
                        }
                    }
                }
                Rounding::TowardsZero => {
                    if $cum_error.unsigned_abs() >= $right.unsigned_abs() {
                        if ($left ^ $right) > 0 {
                            *$cum_error -= $right;
                            q += 1;
                        } else {
                            *$cum_error += $right;
                            q -= 1;
                        }
                    }
                }
                Rounding::AwayFromZero => {
                    if ($left ^ $right) > 0 {
                        if sub_cmp_abs!(*$cum_error, $right) {
                            *$cum_error -= $right;
                            q += 1;
                        }
                    } else {
                        if add_cmp_abs!(*$cum_error, $right) {
                            *$cum_error += $right;
                            q -= 1;
                        }
                    }
                }
            }
            Some(q)
        }
    }
}

pub fn checked_divide_with_cum_error_i8(lhs: i8, rhs: i8,
    rounding: Rounding, cum_error: &mut i8) -> Option<i8>
{
    checked_divide_with_cum_error!(lhs, rhs, rounding, cum_error)
}
pub fn checked_divide_with_cum_error_i16(lhs: i16, rhs: i16,
    rounding: Rounding, cum_error: &mut i16) -> Option<i16>
{
    checked_divide_with_cum_error!(lhs, rhs, rounding, cum_error)
}
pub fn checked_divide_with_cum_error_i32(lhs: i32, rhs: i32,
    rounding: Rounding, cum_error: &mut i32) -> Option<i32>
{
    checked_divide_with_cum_error!(lhs, rhs, rounding, cum_error)
}
pub fn checked_divide_with_cum_error_i64(lhs: i64, rhs: i64,
    rounding: Rounding, cum_error: &mut i64) -> Option<i64>
{
    checked_divide_with_cum_error!(lhs, rhs, rounding, cum_error)
}
pub fn checked_divide_with_cum_error_i128(lhs: i128, rhs: i128,
    rounding: Rounding, cum_error: &mut i128) -> Option<i128>
{
    checked_divide_with_cum_error!(lhs, rhs, rounding, cum_error)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_rounding_fib(a: i32, b: i32, rounding: Rounding) {
        let mut cum_error = 0_i32;
        let mut i0 = 1_i32;
        let mut i1 = 1_i32;
        let mut isum = 0_i32;
        let mut ret = 0_i32;
        while isum.unsigned_abs() < a.unsigned_abs() {
            let ix = i0 + i1;

            ret += checked_divide_with_cum_error!(ix, b, rounding, &mut cum_error).unwrap();
            i0 = i1;
            i1 = ix;
            isum += ix;

            let q = checked_divide!(isum, b, rounding).unwrap();
            let r = isum - q * b;
            assert_eq!(q, ret);
            assert_eq!(r, cum_error);

            if isum.abs() >= a.abs() {
                break;
            }
        }
    }

    fn do_test(a: i32, b: i32) {
        test_rounding_fib(a, b, Rounding::Floor);
        test_rounding_fib(a, b, Rounding::Ceiling);
        test_rounding_fib(a, b, Rounding::Round);
        test_rounding_fib(a, b, Rounding::AwayFromZero);
        test_rounding_fib(a, b, Rounding::TowardsZero);
    }

    fn test(a: i32, b: i32) {
        do_test(a, b);
        do_test(-a, b);
        do_test(a, -b);
        do_test(-a, -b);
    }

    #[test]
    fn many_test() {
        for a in 1..30 {
            for b in 1..30 {
                test(a*3+13, b+7);
                test(a*127+13, b*3+7);
                test(a*3127+17, b*14+8);
                test(a*7127+19, b*115+9);
                test(a*9127+19, b*1116+9);
            }
        }
        println!("done");
    }

    fn do_test_overflow(a: i32, b: i32) {
        let mut cum_error = 0_i32;
        checked_divide_with_cum_error!(a, b, Rounding::Floor, &mut cum_error).unwrap();
        checked_divide_with_cum_error!(a, b, Rounding::Floor, &mut cum_error).unwrap();
        checked_divide_with_cum_error!(a, b, Rounding::Floor, &mut cum_error).unwrap();
        checked_divide_with_cum_error!(a, b, Rounding::Floor, &mut cum_error).unwrap();
        let mut cum_error = 0_i32;
        checked_divide_with_cum_error!(a, b, Rounding::Ceiling, &mut cum_error).unwrap();
        checked_divide_with_cum_error!(a, b, Rounding::Ceiling, &mut cum_error).unwrap();
        checked_divide_with_cum_error!(a, b, Rounding::Ceiling, &mut cum_error).unwrap();
        checked_divide_with_cum_error!(a, b, Rounding::Ceiling, &mut cum_error).unwrap();
        let mut cum_error = 0_i32;
        checked_divide_with_cum_error!(a, b, Rounding::Round, &mut cum_error).unwrap();
        checked_divide_with_cum_error!(a, b, Rounding::Round, &mut cum_error).unwrap();
        checked_divide_with_cum_error!(a, b, Rounding::Round, &mut cum_error).unwrap();
        checked_divide_with_cum_error!(a, b, Rounding::Round, &mut cum_error).unwrap();
        let mut cum_error = 0_i32;
        checked_divide_with_cum_error!(a, b, Rounding::TowardsZero, &mut cum_error).unwrap();
        checked_divide_with_cum_error!(a, b, Rounding::TowardsZero, &mut cum_error).unwrap();
        checked_divide_with_cum_error!(a, b, Rounding::TowardsZero, &mut cum_error).unwrap();
        checked_divide_with_cum_error!(a, b, Rounding::TowardsZero, &mut cum_error).unwrap();
        let mut cum_error = 0_i32;
        checked_divide_with_cum_error!(a, b, Rounding::AwayFromZero, &mut cum_error).unwrap();
        checked_divide_with_cum_error!(a, b, Rounding::AwayFromZero, &mut cum_error).unwrap();
        checked_divide_with_cum_error!(a, b, Rounding::AwayFromZero, &mut cum_error).unwrap();
        checked_divide_with_cum_error!(a, b, Rounding::AwayFromZero, &mut cum_error).unwrap();
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
