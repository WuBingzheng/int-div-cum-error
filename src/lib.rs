#[derive(Clone, Copy, Debug)]
pub enum Rounding {
    Round,
    Floor,
    Ceiling,
    TowardsZero,
    AwayFromZero,
}

pub fn checked_divide(left: i32, right: i32, rounding: Rounding)
    -> Option<i32>
{
    let Some(q) = left.checked_div(right) else {
        return None;
    };

    let remain = left % right;
    if remain == 0 {
        return Some(q);
    }

    Some(match rounding {
        Rounding::Floor => {
            if (left ^ right) > 0 {
                q
            } else {
                q - 1
            }
        }
        Rounding::Ceiling => {
            if (left ^ right) > 0 {
                q + 1
            } else {
                q
            }
        }
        Rounding::Round => {
            if remain.unsigned_abs() >= right.unsigned_abs() / 2 {
                if (left ^ right) > 0 {
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
            if (left ^ right) > 0 {
                q + 1
            } else {
                q - 1
            }
        }
    })
}

// return: abs(a + b) < abs(b)
fn add_cmp_abs(a: i32, b: i32) -> bool {
    match a.checked_add(b) {
        None => false,
        Some(n) => n.unsigned_abs() < b.unsigned_abs(),
    }
}

// return: abs(a - b) < abs(b)
fn sub_cmp_abs(a: i32, b: i32) -> bool {
    match a.checked_sub(b) {
        None => false,
        Some(n) => n.unsigned_abs() < b.unsigned_abs(),
    }
}

pub fn checked_divide_with_cum_error(left: i32, right: i32, rounding: Rounding, cum_error: &mut i32)
    -> Option<i32>
{
    let Some(mut q) = left.checked_div(right) else {
        return None;
    };

    let remain = left % right;
    if remain == 0 {
        return Some(q);
    }

    let Some(tmpsum) = cum_error.checked_add(remain) else {
        if (left ^ right) > 0 {
            *cum_error += remain - right;
            return Some(q + 1);
        } else {
            *cum_error += remain + right;
            return Some(q - 1);
        }
    };
    *cum_error = tmpsum;

    match rounding {
        Rounding::Floor => {
            if (left ^ right) > 0 {
                if cum_error.unsigned_abs() >= right.unsigned_abs() {
                    *cum_error -= right;
                    q += 1;
                }
            } else {
                if add_cmp_abs(*cum_error, right) {
                    *cum_error += right;
                    q -= 1;
                }
            }
        }
        Rounding::Ceiling => {
            if (left ^ right) > 0 {
                if sub_cmp_abs(*cum_error, right) {
                    *cum_error -= right;
                    q += 1;
                }
            } else {
                if cum_error.unsigned_abs() >= right.unsigned_abs() {
                    *cum_error += right;
                    q -= 1;
                }
            }
        }
        Rounding::Round => {
            if cum_error.unsigned_abs() >= right.unsigned_abs() / 2 {
                if (left ^ right) > 0 {
                    *cum_error -= right;
                    q += 1;
                } else {
                    *cum_error += right;
                    q -= 1;
                }
            }
        }
        Rounding::TowardsZero => {
            if cum_error.unsigned_abs() >= right.unsigned_abs() {
                if (left ^ right) > 0 {
                    *cum_error -= right;
                    q += 1;
                } else {
                    *cum_error += right;
                    q -= 1;
                }
            }
        }
        Rounding::AwayFromZero => {
            if (left ^ right) > 0 {
                if sub_cmp_abs(*cum_error, right) {
                    *cum_error -= right;
                    q += 1;
                }
            } else {
                if add_cmp_abs(*cum_error, right) {
                    *cum_error += right;
                    q -= 1;
                }
            }
        }
    }
    Some(q)
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

            ret += checked_divide_with_cum_error(ix, b, rounding, &mut cum_error).unwrap();
            i0 = i1;
            i1 = ix;
            isum += ix;

            let q = checked_divide(isum, b, rounding).unwrap();
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
