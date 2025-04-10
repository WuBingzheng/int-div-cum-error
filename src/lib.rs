#[derive(Clone, Copy, Debug)]
pub enum Rounding {
    Round,
    Floor,
    Ceiling,
    TowardsZero,
    AwayFromZero,
}

pub fn checked_div_rem(left: i32, right: i32, rounding: Rounding)
    -> Option<(i32, i32)>
{
    let Some(q) = left.checked_div(right) else {
        return None;
    };

    let remain = left % right;
    if remain == 0 {
        return Some((q, 0));
    }

    let ret = match rounding {
        Rounding::Floor => {
            if (left ^ right) > 0 {
                (q, remain)
            } else {
                (q - 1, remain + right)
            }
        }
        Rounding::Ceiling => {
            if (left ^ right) > 0 {
                (q + 1, remain - right)
            } else {
                (q, remain)
            }
        }
        Rounding::Round => {
            if remain.abs() * 2 > right.abs() {
                if (left ^ right) > 0 {
                    (q + 1, remain - right)
                } else {
                    (q - 1, remain + right)
                }
            } else {
                (q, remain)
            }
        }
        Rounding::TowardsZero => {
            (q, remain)
        }
        Rounding::AwayFromZero => {
            if (left ^ right) > 0 {
                (q + 1, remain - right)
            } else {
                (q - 1, remain + right)
            }
        }
    };
    Some(ret)
}

pub fn checked_div(left: i32, right: i32, rounding: Rounding, cum_error: &mut i32)
    -> Option<i32>
{
    let Some(mut q) = left.checked_div(right) else {
        return None;
    };

    let remain = left % right;
    if remain == 0 {
        return Some(q);
    }

    let ret = match rounding {
        Rounding::Floor => {
            *cum_error += remain;
            if (left ^ right) > 0 {
                if cum_error.abs() >= right.abs() {
                    *cum_error -= right;
                    q += 1;
                }
            } else {
                if (*cum_error + right).abs() < right.abs() {
                    *cum_error += right;
                    q -= 1;
                }
            }
            q
        }
        Rounding::Ceiling => {
            *cum_error += remain;
            if (left ^ right) > 0 {
                if (*cum_error - right).abs() < right.abs() {
                    *cum_error -= right;
                    q += 1;
                }
            } else {
                if cum_error.abs() >= right.abs() {
                    *cum_error += right;
                    q -= 1;
                }
            }
            /*
            if (left ^ right) > 0 {
                *cum_error -= right;
                q += 1;
            }
            if cum_error.abs() >= right.abs() {
                *cum_error += right;
                q -= 1;
            }
            */
            q
        }
        Rounding::Round => {
            *cum_error += remain;
            if cum_error.abs() * 2 > right.abs() {
                if (left ^ right) > 0 {
                    *cum_error -= right;
                    q += 1;
                } else {
                    *cum_error += right;
                    q -= 1;
                }
            }
            q
        }
        Rounding::TowardsZero => {
            *cum_error += remain;
            if cum_error.abs() >= right.abs() {
                if (left ^ right) > 0 {
                    *cum_error -= right;
                    q += 1;
                } else {
                    *cum_error += right;
                    q -= 1;
                }
            }
            q
        }
        Rounding::AwayFromZero => {
            *cum_error += remain;
            if (left ^ right) > 0 {
                if (*cum_error - right).abs() < right.abs() {
                    *cum_error -= right;
                    q += 1;
                }
            } else {
                if (*cum_error + right).abs() < right.abs() {
                    *cum_error += right;
                    q -= 1;
                }
            }
            q
        }
    };
    Some(ret)
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

            ret += checked_div(ix, b, rounding, &mut cum_error).unwrap();
            i0 = i1;
            i1 = ix;
            isum += ix;

            let (q, r) = checked_div_rem(isum, b, rounding).unwrap();
            if q != ret || r != cum_error {
                println!("fail");
            }

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
        test(14, 3);
        //return;

        test(11000, 1);
        test(11000, 1);
        test(11000, 2);
        test(1100, 1);
        test(1100, 1);
        test(1100, 2);
        test(11000, 1);
        test(11000, 1);
        test(11000, 2);
        test(1100, 1);
        test(1100, 1);
        test(1100, 2);
        test(110001, 1);
        test(110001, 1);
        test(110001, 2);
        test(11001, 1);
        test(11001, 1);
        test(11001, 2);
        test(110001, 1);
        test(110001, 1);
        test(110001, 2);
        test(11001, 1);
        test(11001, 1);
        test(11001, 2);
        test(1000, 1);
        test(1000, 1);
        test(1000, 2);
        test(100, 1);
        test(100, 1);
        test(100, 2);
        test(1000, 1);
        test(1000, 1);
        test(1000, 2);
        test(100, 1);
        test(100, 1);
        test(100, 2);
        test(10001, 1);
        test(10001, 1);
        test(10001, 2);
        test(1001, 1);
        test(1001, 1);
        test(1001, 2);
        test(10001, 1);
        test(10001, 1);
        test(10001, 2);
        test(1001, 1);
        test(1001, 1);
        test(1001, 2);
        test(10001, 1);
        test(10001, 1);
        test(10001, 2);
        test(1001, 1);
        test(1001, 1);
        test(1001, 2);
        println!("done");
    }
}
