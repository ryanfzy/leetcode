pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut i = 0;
    'outer: 
    for c in strs[0].chars() {
        for n in 1..strs.len() {
            if strs[n].chars().nth(i).unwrap_or_default() != c {
                break 'outer;
            }
        }
        i += 1;
    }
    strs[0][0..i].into()
}

// nb
pub fn longest_common_prefix2(strs: Vec<String>) -> String {
    strs.iter()
        .skip(1)
        .fold(strs[0].clone(), |acc, x| {
            acc.chars()
                .zip(x.chars())
                .take_while(|(x, y)| x == y)
                .map(|(x, _)| x)
                .collect()
    })
}

#[test]
pub fn test_longest_common_prefix() {
    assert_eq!("fl".to_string(), longest_common_prefix(vec!["flower".into(), "flow".into(), "flight".into()]));
    assert_eq!("".to_string(), longest_common_prefix(vec!["dog".into(), "racecar".into(), "car".into()]));
}

#[test]
pub fn test_longest_common_prefix2() {
    assert_eq!("fl".to_string(), longest_common_prefix2(vec!["flower".into(), "flow".into(), "flight".into()]));
    assert_eq!("".to_string(), longest_common_prefix2(vec!["dog".into(), "racecar".into(), "car".into()]));
}