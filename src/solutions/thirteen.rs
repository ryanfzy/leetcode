pub fn roman_to_int(s: String) -> i32 {
    use std::collections::HashMap;
    let mut ret = 0;
    let mut prev = 0;
    let map = HashMap::from([
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ]);
    for c in s.chars() {
        let next = *map.get(&c).unwrap();
        if next > prev {
            ret += next - 2 * prev;
        } else {
            ret += next;
        }
        prev = next;
    }
    ret
}

// more efficient
pub fn roman_to_int2(s: String) -> i32 {
    s.chars().rfold(0, |acc, c| {
        acc + match c {
            'I' if acc >= 5 => -1,
            'I' => 1,
            'V' => 5,
            'X' if acc >= 50 => -10,
            'X' => 10,
            'L' => 50,
            'C' if acc >= 500 => -100,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        }
    })
}

#[test]
pub fn test_roman_to_int() {
    assert_eq!(3, roman_to_int("III".to_string()));
    assert_eq!(58, roman_to_int("LVIII".to_string()));
    assert_eq!(1994, roman_to_int("MCMXCIV".to_string()));
}

#[test]
pub fn test_roman_to_int2() {
    assert_eq!(3, roman_to_int2("III".to_string()));
    assert_eq!(58, roman_to_int2("LVIII".to_string()));
    assert_eq!(1994, roman_to_int2("MCMXCIV".to_string()));
}