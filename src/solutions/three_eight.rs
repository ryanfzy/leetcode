pub fn count_and_say(n: i32) -> String {
    if n == 1 {
        "1".to_string()
    } else {
        let ret = count_and_say(n-1);
        let mut new_ret = "".to_string();
        let mut cur_i = 0;
        let mut cur_c = ' ';
        for c in ret.chars() {
            if cur_c != c {
                if cur_i > 0 {
                    new_ret.push_str(&format!("{}{}", cur_i, cur_c));
                }
                cur_i = 1;
                cur_c = c;
            } else {
                cur_i += 1;
            }
        }
        new_ret.push_str(&format!("{}{}", cur_i, cur_c));
        new_ret
    }
}

#[test]
pub fn test_count_and_say() {
    assert_eq!("1".to_string(), count_and_say(1));
    assert_eq!("11".to_string(), count_and_say(2));
    assert_eq!("21".to_string(), count_and_say(3));
    assert_eq!("1211".to_string(), count_and_say(4));
}