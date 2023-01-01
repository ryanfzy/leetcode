pub fn generate_parenthesis(n: i32) -> Vec<String> {
    fn _generate(s: &mut Vec<char>, open: i32, close: i32) -> Vec<String> {
        let mut result = vec![];
        if open == 0 && close == 0 {
            return vec![s.iter().collect::<String>()];
        }
        if open > 0 {
            s.push('(');
            result.append(&mut _generate(s, open-1, close+1));
            s.pop();
        }
        if close > 0 {
            s.push(')');
            result.append(&mut _generate(s, open, close-1));
            s.pop();
        }
        result
    }
    let mut result = vec![];
    _generate(&mut result, n, 0)
}

#[test]
pub fn test_generate_parenthesis() {
    assert_eq!(vec!["((()))", "(()())", "(())()", "()(())", "()()()"], generate_parenthesis(3));
}