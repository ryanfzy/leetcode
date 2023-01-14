pub fn longest_valid_parentheses(s: String) -> i32 {
    let mut result = vec![];
    for c in s.chars() {
        if c == '(' {
            result.push(0);
        } else {
            let mut new_r = 0;
            let mut cur = 1;
            while let Some(r) = result.pop() {
                if r == -1 {
                    result.push(-1);
                    break;
                } else if r == 0 {
                    if cur > 0 {
                        new_r += 2;
                        cur -= 1;
                    } else {
                        result.push(0);
                        break;
                    }
                } else {
                    new_r += r;
                }
            }
            result.push(new_r);
            if cur > 0 {
                result.push(-1);
            }
        }
    }
    result.push(0);
    result.into_iter().max().unwrap()
}

// very smart
pub fn longest_valid_parentheses2(s: String) -> i32 {
    let mut valid = vec![0; s.len() + 1];
    let s = s.as_bytes();
    for (i, &c) in s.iter().enumerate() {
        if c == b'(' {
            continue;
        }
        if let Some(idx) = i.checked_sub(valid[i] + 1) {
            if s[idx] == b'(' {
                valid[i + 1] = valid[i] + 2 + valid[idx];
            }
        }
    }
    valid.into_iter().max().unwrap_or(0) as i32
}

#[test]
pub fn test_longest_valid_parentheses() {
    assert_eq!(0, longest_valid_parentheses("".to_string()));
    assert_eq!(0, longest_valid_parentheses("(".to_string()));
    assert_eq!(2, longest_valid_parentheses("(()".to_string()));
    assert_eq!(4, longest_valid_parentheses(")()())".to_string()));
    assert_eq!(2, longest_valid_parentheses("()(()".to_string()));
    assert_eq!(6, longest_valid_parentheses("()(())".to_string()));
    assert_eq!(6, longest_valid_parentheses(")(())))(())())".to_string()));
    assert_eq!(4, longest_valid_parentheses("(()))())(".to_string()));

    assert_eq!(6, longest_valid_parentheses2("()(())".to_string()));
}