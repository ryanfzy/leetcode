pub fn str_str(haystack: String, needle: String) -> i32 {
    let h = haystack.as_bytes();
    let n = needle.as_bytes();
    let mut i = 0;
    let mut j = 0;
    while i < h.len() {
        if j == n.len() {
            return (i-n.len()) as i32;
        } else if h[i] != n[j] {
            i -= j;
            j = 0;
        } else {
            j += 1;
        }
        i += 1;
    }
    if j == n.len() {
        return (i-n.len()) as i32;
    }
    -1
}

#[test]
pub fn test_str_str() {
    assert_eq!(0, str_str("sadbutsad".to_string(), "sad".to_string()));
    assert_eq!(3, str_str("butsad".to_string(), "sad".to_string()));
    assert_eq!(-1, str_str("leetcode".to_string(), "leeto".to_string()));
    assert_eq!(4, str_str("mississippi".to_string(), "issip".to_string()));
}