pub fn reverse(x: i32) -> i32 {
    let mut result: i32 = 0;
    let mut input = x.abs();
    while input > 0 {
        let r = input % 10;
        if let Some(ret) = result.checked_mul(10) {
            result = ret + r;
            input /= 10;
        } else {
            result = 0;
            break;
        }
    }
    x.signum() * result
}

#[test]
pub fn test_reverse() {
    assert_eq!(321, reverse(123));
    assert_eq!(-321, reverse(-123));
}