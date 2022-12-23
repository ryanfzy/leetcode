pub fn letter_combinations(digits: String) -> Vec<String> {
    use std::collections::HashMap;
    if digits.len() == 0 {
        return vec![];
    }
    let mut result = vec![];
    let mut result2 = vec![];
    let m = HashMap::from([
        ('2', "abc"),
        ('3', "def"),
        ('4', "ghi"),
        ('5', "jkl"),
        ('6', "mno"),
        ('7', "pqrs"),
        ('8', "tuv"),
        ('9', "wxyz")
    ]);
    for c in digits.chars() {
        if let Some(v) = m.get(&c) {
            if result.len() == 0 {
                for vc in v.chars() {
                    result.push(vec![vc]);
                }
            } else {
                for r in result {
                    for vc in v.chars() {
                        let mut t = r.clone();
                        t.push(vc);
                        result2.push(t);
                    }
                }
                result = result2;
                result2 = vec![];
            }
        }
    }
    result.into_iter().map(|v| v.into_iter().collect()).collect()
}

// more efficient
pub fn letter_combinations2(digits: String) -> Vec<String> {
    if digits.len() == 0 {
        return vec![];
    }
    use std::collections::HashMap;
    let m = HashMap::from([
        (b'2', vec!['a', 'b', 'c']),
        (b'3', vec!['d', 'e', 'f']),
        (b'4', vec!['g', 'h', 'i']),
        (b'5', vec!['j', 'k', 'l']),
        (b'6', vec!['m', 'n', 'o']),
        (b'7', vec!['p', 'q', 'r', 's']),
        (b'8', vec!['t', 'u', 'v']),
        (b'9', vec!['w', 'x', 'y', 'z'])
    ]);
    digits
        .as_bytes()
        .iter()
        .fold(vec![vec![]], |acc, d| {
            acc.iter().flat_map(|v| {
                std::iter::repeat(v)
                    .zip(m.get(d).unwrap().clone())
                    .map(|(v, b)| { let mut t = v.clone(); t.push(b); t })
                    .collect::<Vec<_>>()
            }).collect()
        })
        .into_iter()
        .map(|v| v.into_iter().collect())
        .collect()
}

#[test]
pub fn test_letter_combinations() {
    assert_eq!(vec!["a", "b", "c"], letter_combinations("2".to_string()));
    assert_eq!(vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"], letter_combinations("23".to_string()));
}

#[test]
pub fn test_letter_combinations2() {
    assert_eq!(vec!["a", "b", "c"], letter_combinations2("2".to_string()));
    assert_eq!(vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"], letter_combinations2("23".to_string()));
}