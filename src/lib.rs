#[derive(Clone, Copy, Debug)]
pub enum Rounding {
    Round,
    Floor,
    Ceiling,
    TowardsZero,
    AwayFromZero,
}

fn mydiv(left: i32, right: i32, rounding: Rounding, cum_error: &mut i32) -> Option<i32>
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

fn test_rounding_fab(a: i32, b: i32, begin: i32, rounding: Rounding) {
    let mut cum_error = 0;
    let mut i0 = 1;
    let mut i1 = begin;
    let mut isum = 0;
    let mut ret = 0;
    loop {
        let ix = i0 + i1;
        if (ix + isum).abs() > a.abs() {
            break;
        }
        ret += mydiv(ix, b, rounding, &mut cum_error).unwrap();
        //println!(" -- {ix} / {b} = {ret} ... {cum_error}");

        i0 = i1;
        i1 = ix;
        isum += ix;
    }

    let mut error0 = 0;
    let r0 = mydiv(isum, b, rounding, &mut error0).unwrap();

    if ret !=r0 || cum_error != error0 {
      println!("{a}/{b} = {ret},{cum_error},   {r0},{error0} | isum={isum} {:?}", rounding);
    }
}
fn test_rounding_mul(a: i32, b: i32, rounding: Rounding) {
    let mut cum_error = 0;
    let mut ret = 0;
    for _  in 0..b.unsigned_abs() {
        ret += mydiv(a, b, rounding, &mut cum_error).unwrap();
    }
    if ret.abs() != a.abs() || cum_error != 0 {
      println!("{a}/{b} = {ret},{cum_error}  {:?}", rounding);
    }
}
fn do_test(a: i32, b: i32, begin: i32) {
    test_rounding_mul(a, b, Rounding::Floor);
    test_rounding_mul(a, b, Rounding::Ceiling);
    test_rounding_mul(a, b, Rounding::Round);
    test_rounding_mul(a, b, Rounding::AwayFromZero);
    test_rounding_mul(a, b, Rounding::TowardsZero);
    test_rounding_fab(a, b, begin, Rounding::Floor);
    test_rounding_fab(a, b, begin, Rounding::Ceiling);
    test_rounding_fab(a, b, begin, Rounding::Round);
    test_rounding_fab(a, b, begin, Rounding::AwayFromZero);
    test_rounding_fab(a, b, begin, Rounding::TowardsZero);
}
fn test(a: i32, b: i32, begin: i32) {
    do_test(a, b, begin);
    do_test(-a, b, begin);
    do_test(a, -b, begin);
    do_test(-a, -b, begin);
}
fn main() {
    test(14, 3, 1);
    //return;

    test(11000, 13, 3);
    test(11000, 17, 3);
    test(11000, 217, 3);
    test(1100, 13, 3);
    test(1100, 17, 3);
    test(1100, 217, 3);
    test(11000, 13, 1);
    test(11000, 17, 1);
    test(11000, 217, 1);
    test(1100, 13, 1);
    test(1100, 17, 1);
    test(1100, 217, 1);
    test(110001, 13, 3);
    test(110001, 17, 3);
    test(110001, 217, 3);
    test(11001, 13, 3);
    test(11001, 17, 3);
    test(11001, 217, 3);
    test(110001, 13, 1);
    test(110001, 17, 1);
    test(110001, 217, 1);
    test(11001, 13, 1);
    test(11001, 17, 1);
    test(11001, 217, 1);
    test(1000, 13, 3);
    test(1000, 17, 3);
    test(1000, 217, 3);
    test(100, 13, 3);
    test(100, 17, 3);
    test(100, 217, 3);
    test(1000, 13, 1);
    test(1000, 17, 1);
    test(1000, 217, 1);
    test(100, 13, 1);
    test(100, 17, 1);
    test(100, 217, 1);
    test(10001, 13, 3);
    test(10001, 17, 3);
    test(10001, 217, 3);
    test(1001, 13, 3);
    test(1001, 17, 3);
    test(1001, 217, 3);
    test(10001, 13, 1);
    test(10001, 17, 1);
    test(10001, 217, 1);
    test(1001, 13, 1);
    test(1001, 17, 1);
    test(1001, 217, 1);
    test(10001, 12, 1);
    test(10001, 16, 1);
    test(10001, 212, 1);
    test(1001, 12, 1);
    test(1001, 16, 1);
    test(1001, 212, 1);
    println!("done");
}
/*
fn test_rounding(a: i32, b: i32, rounding: Rounding) {
    println!("{a} / {b} = {:?}", rounding);

    let mut cum_error = 0;
    let mut sum = 0;
    for _ in 0 .. b.unsigned_abs() {
        //let ret = mydiv(a, b, rounding, &mut cum_error).unwrap();
        let ret = divide(a, b, rounding, &mut cum_error).unwrap();
        sum += ret;
        println!("   = {},{},  {}", ret, cum_error, sum);
    }
}

fn test(a: i32, b: i32) {
    test_rounding(a, b, Rounding::Floor);
    test_rounding(a, b, Rounding::Ceiling);
}

fn main() {
    test(10, 3);
    test(-10, 3);
    test(10, -3);
    test(-10, -3);
    println!("-");

    test(20, 3);
    test(-20, 3);
    test(20, -3);
    test(-20, -3);
    println!("-");

    test(10, 6);
    test(-10, 6);
    test(10, -6);
    test(-10, -6);
    println!("-");

    test(11, 5);
    test(-11, 5);
    test(11, -5);
    test(-11, -5);
    println!("-");

    test(14, 5);
    test(-14, 5);
    test(14, -5);
    test(-14, -5);
}
    */
