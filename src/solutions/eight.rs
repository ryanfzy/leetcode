pub fn my_atoi(s: String) -> i32 {
    let chars = s.as_bytes();
    let mut start = 0;
    for c in chars {
        if *c == b' ' {
            start += 1;
        } else {
            break;
        }
    }
    if chars.len() == 0 || start == chars.len() {
        return 0;
    }
    let mut signnum = 1;
    if chars[start] == b'-' {
        signnum = -1;
        start += 1;
    } else if chars[start] == b'+' {
        start += 1;
    }
    let mut r: i32 = 0;
    for c in chars[start..].iter() {
        if *c >= b'0' && *c <= b'9' {
            let i = *c - b'0';
            if let Some(x) = r.checked_mul(10).and_then(|r| r.checked_add(i as i32)) {
                r = x;
            } else {
                r = i32::MAX;
                if signnum == -1 {
                    r = i32::MIN;
                    signnum = 1;
                }
                break;
            }
        } else {
            break;
        }
    }
    signnum * r
}

pub fn my_atoi2(s: String) -> i32 {
    let mut chars = s.chars().skip_while(|&c| c == ' ').peekable();
    let mut sign: i32 = 1;
    if let Some(n) = chars.peek() {
        if *n == '-' {
            sign = -1;
            chars.next();
        } else if *n == '+' {
            chars.next();
        }
    }
    chars.take_while(|&c| c >= '0' && c <= '9')
        .map(|c| c as i32 - '0' as i32)
        .try_fold(0i32, |acc, i| acc.checked_mul(10).and_then(|acc| acc.checked_add(i)))
        .map(|i| i * sign)
        .unwrap_or(if sign > 0 { i32::MAX } else { i32::MIN })
}

#[test]
pub fn test_my_atoi() {
    assert_eq!(4193, my_atoi("4193 with words".to_string()));
    assert_eq!(-42, my_atoi("    -42".to_string()));
    assert_eq!(-2147483648, my_atoi("-91283472332".to_string()));
    assert_eq!(0, my_atoi("".to_string()));
    assert_eq!(0, my_atoi("  ".to_string()));
    assert_eq!(0, my_atoi("  ab".to_string()));
}

#[test]
pub fn test_my_atoi2() {
    assert_eq!(4193, my_atoi2("4193 with words".to_string()));
    assert_eq!(-42, my_atoi2("    -42".to_string()));
    assert_eq!(-2147483648, my_atoi2("-91283472332".to_string()));
    assert_eq!(0, my_atoi2("".to_string()));
    assert_eq!(0, my_atoi2("  ".to_string()));
    assert_eq!(0, my_atoi2("  ab".to_string()));
}