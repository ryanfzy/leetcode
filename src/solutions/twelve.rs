pub fn int_to_roman(num: i32) -> String {
    let mut p = vec![];
    let mut r = num;
    let mut n = r / 1000;
    if n > 0 {
        for _ in 0..n {
            p.push("M");
        }
        r -= n * 1000;
    }
    if r >= 900 {
        p.push("CM");
        r -= 900;
    }
    if r >= 500 {
        p.push("D");
        r -= 500;
    }
    if r >= 400 {
        p.push("CD");
        r -= 400;
    }
    n = r / 100;
    if n > 0 {
        for _ in 0..n {
            p.push("C");
        }
        r -= n * 100;
    }
    if r >= 90 {
        p.push("XC");
        r -= 90;
    }
    if r >= 50 {
        p.push("L");
        r -= 50;
    }
    if r >= 40 {
        p.push("XL");
        r -= 40;
    }
    n = r / 10;
    if n > 0 {
        for _ in 0..n {
            p.push("X")
        }
        r -= n * 10;
    }
    if r == 9 {
        p.push("IX");
        r -= 9;
    }
    if r >= 5 {
        p.push("V");
        r -= 5;
    }
    if r >= 4 {
        p.push("IV");
        r -= 4;
    }
    if r > 0 {
        for _ in 0..r {
            p.push("I");
        }
    }
    p.concat()
}

pub fn int_to_roman2(num: i32) -> String {
    let mut r = Vec::new();
    let ones = ["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"];
    let tens = ["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"];
    let hundreds = ["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"];
    let thousands = ["", "M", "MM", "MMM"];
    r.push(thousands[(num / 1000 % 10) as usize]);
    r.push(hundreds[(num / 100 % 10) as usize]);
    r.push(tens[(num / 10 % 10) as usize]);
    r.push(ones[(num % 10) as usize]);
    return r.concat();
}

pub fn int_to_roman3(num: i32) -> String {
    let ones = ["I", "X", "C", "M"];
    let fives = ["V", "L", "D"];
    let mut digits = Vec::new();
    let mut n = num;
    while n > 0 {
        digits.push(n % 10);
        n /= 10;
    }
    digits.into_iter().enumerate().rev().map(|(i, d)| {
        match d {
            1 => vec![ones[i]],
            2 => vec![ones[i], ones[i]],
            3 => vec![ones[i], ones[i], ones[i]],
            4 => vec![ones[i], fives[i]],
            5 => vec![fives[i]],
            6 => vec![fives[i], ones[i]],
            7 => vec![fives[i], ones[i], ones[i]],
            8 => vec![fives[i], ones[i], ones[i], ones[i]],
            9 => vec![ones[i], ones[i+1]],
            _ => vec![]
        }
    }).flatten().collect()
}

#[test]
pub fn test_int_roman() {
    assert_eq!("III".to_string(), int_to_roman(3));
    assert_eq!("LVIII".to_string(), int_to_roman(58));
    assert_eq!("MCMXCIV".to_string(), int_to_roman(1994));
    assert_eq!("CDI".to_string(), int_to_roman(401));
}

#[test]
pub fn test_int_roman2() {
    assert_eq!("III".to_string(), int_to_roman2(3));
    assert_eq!("LVIII".to_string(), int_to_roman2(58));
    assert_eq!("MCMXCIV".to_string(), int_to_roman2(1994));
    assert_eq!("CDI".to_string(), int_to_roman2(401));
}

#[test]
pub fn test_int_roman3() {
    assert_eq!("III".to_string(), int_to_roman3(3));
    assert_eq!("LVIII".to_string(), int_to_roman3(58));
    assert_eq!("MCMXCIV".to_string(), int_to_roman3(1994));
    assert_eq!("CDI".to_string(), int_to_roman3(401));
}