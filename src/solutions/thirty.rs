// slow because of the permutation
pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
    use std::collections::HashSet;
    fn _find(s: &str, words: Vec<String>, per: String, result: &mut HashSet<i32>) {
        if words.len() == 0 {
            let mut start = 0;
            let mut t = &s[0..];
            while let Some(i) = t.find(&per) {
                let cur = i + start;
                result.insert(cur as i32);
                if t.len() == per.len() {
                    break;
                }
                t = &s[cur+1..];
                start = cur + 1;
            }
        } else {
            for i in 0..words.len() {
                let mut new_words = words.clone();
                new_words.remove(i);
                let new_per = per.clone() + &words[i];
                _find(s, new_words, new_per, result);
            }
        }
    }
    let mut result = HashSet::new();
    _find(&s, words, "".to_string(), &mut result);
    let mut r: Vec<i32> = result.into_iter().collect();
    r.sort();
    r
}

// faster without permutation
pub fn find_substring2(s: String, words: Vec<String>) -> Vec<i32> {
    use std::collections::HashMap;

    let map = words.iter().fold(HashMap::new(), |mut m, w| {
        let e = m.entry(w.as_str()).or_insert(0);
        *e += 1;
        m
    });

    let mut result = vec![];
    let word_len = words[0].len();
    let words_len = words.len() * word_len;

    'outer: for i in 0..s.len() - words_len + 1 {
        let mut m = map.clone();
        for j in (i..(i+words_len)).step_by(word_len) {
            if let Some(w) = m.get_mut(&s[j..j+word_len]) {
                *w -= 1;
                if *w < 0 {
                    continue 'outer;
                }
            } else {
                continue 'outer;
            }
        }
        result.push(i as i32);
    }
    result
}

#[test]
pub fn test_find_substring() {
    assert_eq!(vec![0, 9], find_substring("barfoothefoobarman".to_string(), vec!["foo".to_string(), "bar".to_string()]));
    assert_eq!(vec![0, 3, 6], find_substring("foobarfoobar".to_string(), vec!["foo".to_string(), "bar".to_string()]));
    assert_eq!(vec![0, 1], find_substring("aaa".to_string(), vec!["a".to_string(), "a".to_string()]));
}

#[test]
pub fn test_find_substring2() {
    assert_eq!(vec![0, 9], find_substring2("barfoothefoobarman".to_string(), vec!["foo".to_string(), "bar".to_string()]));
    assert_eq!(vec![0, 3, 6], find_substring2("foobarfoobar".to_string(), vec!["foo".to_string(), "bar".to_string()]));
    assert_eq!(vec![0, 1], find_substring2("aaa".to_string(), vec!["a".to_string(), "a".to_string()]));
}