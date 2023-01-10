pub fn divide(dividend: i32, divisor: i32) -> i32 {
    match (dividend, divisor) {
        (i32::MIN, -1) => return i32::MAX,
        (i32::MAX, -1) => return i32::MIN + 1,
        (i32::MAX, 1) => return i32::MAX,
        (i32::MIN, 1) => return i32::MIN,
        (i32::MIN, i32::MIN) => return 1,
        (i32::MAX, i32::MAX) => return 1,
        (i32::MIN, i32::MAX) => return -1,
        (_, i32::MAX) => return 0,
        (_, i32::MIN) => return 0,
        (mut a, mut b) => {
            let sign = a.signum() * b.signum();
            b = b.abs();
            let mut r = 0;
            if a < 0 {
                while a < 0 {
                    a += b;
                    r += 1;
                }
                if a > 0 {
                    r -= 1;
                }
            } else {
                while a >= b {
                    a -= b;
                    r += 1;
                }
            }
            sign * r
        }
    }
}

// much faster
pub fn divide2(dividend: i32, divisor: i32) -> i32 {
    match (dividend, divisor) {
        (i32::MIN, -1) => return i32::MAX,
        (i32::MAX, -1) => return i32::MIN + 1,
        (i32::MAX, 1) => return i32::MAX,
        (i32::MIN, 1) => return i32::MIN,
        (i32::MIN, i32::MIN) => return 1,
        (i32::MAX, i32::MAX) => return 1,
        (i32::MIN, i32::MAX) => return -1,
        (_, i32::MAX) => return 0,
        (_, i32::MIN) => return 0,
        (mut a, mut b) => {
            let is_neg = (a < 0) ^ (b < 0);
            a = if a > 0 { -a } else { a };
            b = if b > 0 { -b } else { b };
            let mut r = 0;
            for s in (0..b.leading_ones()).rev() {
                if a <= (b << s) {
                    a -= b << s;
                    r += -1 << s;
                }
            }
            if is_neg {
                r
            } else {
                -r
            }
        }
    }
}

#[test]
pub fn test_divide() {
    assert_eq!(3, divide(10, 3));
    assert_eq!(-2, divide(7, -3));
    assert_eq!(2147483647, divide(-2147483648, -1));
    assert_eq!(-2147483648, divide(-2147483648, 1));
    assert_eq!(-1, divide(1, -1));
    //assert_eq!(-1073741824, divide(-2147483648, 2));
    //assert_eq!(0, divide(1038925803, -2147483648));
    //assert_eq!(-1, divide(-2147483648, 2147483647));
}

#[test]
pub fn test_divide2() {
    assert_eq!(3, divide2(10, 3));
    assert_eq!(-2, divide2(7, -3));
    assert_eq!(2147483647, divide2(-2147483648, -1));
    assert_eq!(-2147483648, divide2(-2147483648, 1));
    assert_eq!(-1, divide2(1, -1));
    assert_eq!(-1073741824, divide2(-2147483648, 2));
    assert_eq!(0, divide2(1038925803, -2147483648));
    assert_eq!(-1, divide2(-2147483648, 2147483647));
}