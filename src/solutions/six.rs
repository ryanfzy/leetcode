pub fn zigzap_conversion(s: String, num_rows: i32) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut result = vec![];
    let step = if num_rows > 2 { num_rows as usize * 2 - 2 } else { num_rows as usize };
    let rows = num_rows as usize - 1;
    let first_indexes: Vec<usize> = (0..chars.len()+step).step_by(step).collect();
    let mut indexes = vec![];
    for i in &first_indexes {
        indexes.push(*i);
    }
    for n in 1..rows {
        indexes.push(first_indexes[0]+n);
        for i in &first_indexes[1..] {
            indexes.push(*i - n);
            indexes.push(*i + n);
        }
    }
    if num_rows > 1 {
        for i in &first_indexes {
            indexes.push(*i + rows);
        }
    }
    for i in indexes.iter().filter(|&x| *x < chars.len()) {
        result.push(chars[*i]);
    }
    result.iter().collect()
}

pub fn zigzap_conversion2(s: String, num_rows: i32) -> String {
    let mut zigzags: Vec<_> = (0..num_rows)
        .chain((1..num_rows-1).rev())
        .cycle()
        .zip(s.chars())
        .collect();
    zigzags.sort_by_key(|&(row, _)| row);
    zigzags.into_iter()
        .map(|(_, c)| c)
        .collect()
}

#[test]
pub fn test_zigzap_conversion() {
    assert_eq!("PAHNAPLSIIGYIR".to_string(), zigzap_conversion("PAYPALISHIRING".to_string(), 3));
    assert_eq!("PINALSIGYAHRPI".to_string(), zigzap_conversion("PAYPALISHIRING".to_string(), 4));
    assert_eq!("A".to_string(), zigzap_conversion("A".to_string(), 1));
    assert_eq!("".to_string(), zigzap_conversion("".to_string(), 1));
    assert_eq!("AB".to_string(), zigzap_conversion("AB".to_string(), 1));
    assert_eq!("ABDC".to_string(), zigzap_conversion("ABCD".to_string(), 3));
    assert_eq!("AB".to_string(), zigzap_conversion("AB".to_string(), 2));
}

#[test]
pub fn test_zigzap_conversion2() {
    assert_eq!("PAHNAPLSIIGYIR".to_string(), zigzap_conversion2("PAYPALISHIRING".to_string(), 3));
    assert_eq!("PINALSIGYAHRPI".to_string(), zigzap_conversion2("PAYPALISHIRING".to_string(), 4));
    assert_eq!("A".to_string(), zigzap_conversion2("A".to_string(), 1));
    assert_eq!("".to_string(), zigzap_conversion2("".to_string(), 1));
    assert_eq!("AB".to_string(), zigzap_conversion2("AB".to_string(), 1));
    assert_eq!("ABDC".to_string(), zigzap_conversion2("ABCD".to_string(), 3));
    assert_eq!("AB".to_string(), zigzap_conversion2("AB".to_string(), 2));
}