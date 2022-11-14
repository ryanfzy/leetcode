use std::str;

pub fn longest_palindrome(s: String) -> String {
    for size in (1..s.len()+1).rev() {
        for w in s.as_bytes().windows(size) {
            if w.iter().zip(w.iter().rev()).all(|(l, r)| l == r) {
                return str::from_utf8(w).unwrap().to_string();
            }
        }
    }
    "".to_string()
}

pub fn longest_palindrome2(s: String) -> String {
    fn _check(m: usize, n: usize, bytes: &[u8]) -> (usize, usize) {
        let mut x = 0;
        let mut y = 0;
        for (a, b) in (0..=m).rev().zip(n..bytes.len()) {
            if bytes[a] == bytes[b] {
                x = a;
                y = b;
            } else {
                break;
            }
        }
        (x, y)
    }
    let bytes = s.as_bytes();
    let len = bytes.len();
    let mut x = 0;
    let mut y = 0;
    for i in 1..len {
        let m = i - 1;
        let mut n = i;
        let (mut a, mut b) = _check(m, n, bytes);
        if b - a  > y - x {
            x = a;
            y = b;
        }
        if n < len-1 {
            n = i + 1;
            (a, b) = _check(m, n, bytes);
            if b - a > y - x {
                x = a;
                y = b;
            }
        }
    }
    str::from_utf8(&bytes[x..=y]).unwrap().to_string()
}

#[test]
pub fn test_longest_palindrome() {
    assert_eq!("bab".to_string(), longest_palindrome("babad".to_string()));
    assert_eq!("bb".to_string(), longest_palindrome("bb".to_string()));
    assert_eq!("a".to_string(), longest_palindrome("a".to_string()));
}

#[test]
pub fn test_longest_palindrome2() {
    assert_eq!("aba".to_string(), longest_palindrome2("caba".to_string()));
    assert_eq!("cc".to_string(), longest_palindrome2("ccd".to_string()));
    assert_eq!("bb".to_string(), longest_palindrome2("cbbd".to_string()));
    assert_eq!("bab".to_string(), longest_palindrome2("babad".to_string()));
    assert_eq!("bb".to_string(), longest_palindrome2("bb".to_string()));
    assert_eq!("a".to_string(), longest_palindrome2("a".to_string()));
    assert_eq!("a".to_string(), longest_palindrome2("ac".to_string()));
}