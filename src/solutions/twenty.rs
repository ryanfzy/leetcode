pub fn is_valid(s: String) -> bool {
    let mut v = vec![];
    for c in s.chars() {
        if c == '(' || c == '{' || c == '[' {
            v.push(c);
        } else {
            if let Some(l) = v.pop() {
                if c == ')' && l != '(' || c == '}' && l != '{' || c == ']' && l != '[' {
                    return false;
                }
            } else {
                return false;
            }
        }
    }
    v.is_empty()
}

#[test]
pub fn test_is_valid() {
    assert_eq!(true, is_valid("()".to_string()));
    assert_eq!(true, is_valid("()[]{}".to_string()));
    assert_eq!(false, is_valid("(]".to_string()));
    assert_eq!(false, is_valid("[".to_string()));
}