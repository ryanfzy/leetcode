pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }
    let mut a = x;
    let mut b = 0;
    while a >= 10 {
        b *= 10;
        b += a % 10;
        a /= 10;
    }
    if a > 0 {
        b *= 10;
        b += a % 10;
    }
    b == x
}

#[test]
fn test_is_palindrome() {
    assert_eq!(true, is_palindrome(121));
    assert_eq!(false, is_palindrome(-121));
    assert_eq!(false, is_palindrome(10));
}