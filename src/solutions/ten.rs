pub fn is_match(s: String, p: String) -> bool {
    let str = s.as_bytes();
    let pat = p.as_bytes();
    let mut s_i = 0;
    let mut p_i = 0;
    while p_i < pat.len() {
        if str[s_i] == pat[p_i] || pat[p_i] == b'.' {
            s_i += 1;
            p_i += 1;
            if p_i < pat.len() && pat[p_i] == b'*' {
                p_i -= 1;
            }
        } else if p_i + 1 < pat.len() && pat[p_i+1] == b'*' {
            p_i += 2;
        } else {
            break;
        }
        if s_i == str.len() && (p_i == pat.len() || p_i + 1 == pat.len() - 1 && pat[p_i+1] == b'*') {
            return true;
        }
    }
    false
}

// very slow: 1141ms
pub fn is_match2(s: String, p: String) -> bool {
    pub fn _is_match(s: &[u8], p: &[u8]) -> bool {
        if s.len() == 0 && p.len() == 0 {
            return true;
        }
        if p.len() == 0 {
            return false;
        }
        if s.len() > 0 && (s[0] == p[0] || p[0] == b'.') {
            if _is_match(&s[1..], &p[1..]) {
                return true;
            }
            if p.len() >= 2 && p[1] == b'*' {
                if _is_match(&s[1..], p) {
                    return true;
                }
                if _is_match(&s[1..], &p[2..]) {
                    return true;
                }
            }
        }
        if p.len() >= 2 && p[1] == b'*' {
            return _is_match(s, &p[2..]);
        }
        false
    }
    _is_match(s.as_bytes(), p.as_bytes())
}

// fast: 17ms
pub fn is_match3(s: String, p: String) -> bool {
    pub fn _is_match(s: &[u8], p: &[u8]) -> bool {
        match (s, p) {
            ([x, m @ ..], [y, b'*', n @ ..]) => {
                if x == y || *y == b'.' {
                    return _is_match(m, p) || _is_match(s, n);
                }
                _is_match(s, n)
            }
            ([x, m @ ..], [y, n @ ..]) => {
                if x == y || *y == b'.' {
                    return _is_match(m, n);
                }
                false
            }
            ([], [_, b'*', n @ ..]) => {
                _is_match(s, n)
            }
            ([], []) => true,
            _ => false,
        }
    }
    _is_match(s.as_bytes(), p.as_bytes())
}

#[test]
pub fn test_is_match() {
    assert_eq!(true, is_match("aa".to_string(), "a*".to_string()));
    assert_eq!(false, is_match("aa".to_string(), "a".to_string()));
    assert_eq!(true, is_match("aab".to_string(), "c*a*b".to_string()));
    //assert_eq!(true, is_match("aaa".to_string(), "a*a".to_string()));
}

#[test]
pub fn test_is_match2() {
    assert_eq!(true, is_match2("aa".to_string(), "aa".to_string()));
    assert_eq!(false, is_match2("aa".to_string(), "aaa".to_string()));
    assert_eq!(false, is_match2("aaa".to_string(), "aa".to_string()));
    assert_eq!(true, is_match2("aa".to_string(), "a.".to_string()));
    assert_eq!(true, is_match2("aa".to_string(), "a*".to_string()));
    assert_eq!(true, is_match2("aab".to_string(), "c*a*b".to_string()));
    assert_eq!(true, is_match2("aaa".to_string(), "a*a".to_string()));
    assert_eq!(false, is_match2("mississippi".to_string(), "mis*s*p*.".to_string()));
    assert_eq!(true, is_match2("a".to_string(), "ab*".to_string()));
}

#[test]
pub fn test_is_match3() {
    assert_eq!(true, is_match3("aa".to_string(), "aa".to_string()));
    assert_eq!(false, is_match3("aa".to_string(), "aaa".to_string()));
    assert_eq!(false, is_match3("aaa".to_string(), "aa".to_string()));
    assert_eq!(true, is_match3("aa".to_string(), "a.".to_string()));
    assert_eq!(true, is_match3("aa".to_string(), "a*".to_string()));
    assert_eq!(true, is_match3("aab".to_string(), "c*a*b".to_string()));
    assert_eq!(true, is_match3("aaa".to_string(), "a*a".to_string()));
    assert_eq!(false, is_match3("mississippi".to_string(), "mis*s*p*.".to_string()));
    assert_eq!(true, is_match3("a".to_string(), "ab*".to_string()));
    assert_eq!(true, is_match3("bbbba".to_string(), ".*a*a".to_string()));
}