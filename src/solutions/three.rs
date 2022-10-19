use std::cmp::max;
use std::collections::HashMap;

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut start = 0;
    let mut end = 0;
    let mut longest = 0;
    let s_vec: Vec<char> = s.chars().collect();
    for i in 0..s_vec.len() {
        for j in start..end {
            if s_vec[j] == s_vec[i] {
                start = j + 1;
                break;
            }
        }
        end += 1;
        longest = max(longest, end - start);
    }
    longest as i32
}

// same logic but much improved version
pub fn length_of_longest_substring2(s: String) -> i32 {
    let mut start = -1;
    let mut end = 0;
    let mut longest = 0;
    let mut m = HashMap::new();
    for c in s.chars() {
        if let Some(last) = m.insert(c, end) {
            start = max(start, last);
        }
        longest = max(longest, end - start);
        end += 1;
    }
    longest
}

#[test]
pub fn test_length_of_longest_substring() {
    assert_eq!(length_of_longest_substring("abcabcbb".to_string()), 3);
    assert_eq!(length_of_longest_substring("bbbbb".to_string()), 1);
    assert_eq!(length_of_longest_substring("pwwkew".to_string()), 3);
    assert_eq!(length_of_longest_substring("aab".to_string()), 2);
}

#[test]
pub fn test_length_of_longest_substring2() {
    assert_eq!(length_of_longest_substring2("abcabcbb".to_string()), 3);
    assert_eq!(length_of_longest_substring2("bbbbb".to_string()), 1);
    assert_eq!(length_of_longest_substring2("pwwkew".to_string()), 3);
    assert_eq!(length_of_longest_substring2("aab".to_string()), 2);
}