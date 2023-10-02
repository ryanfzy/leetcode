fn _combination_sum(candidates: &[i32], target: i32, cur: Vec<i32>, result: &mut Vec<Vec<i32>>) {
    let sum: i32 = cur.iter().sum();
    if sum == target {
        result.push(cur);
    } else if sum < target {
        for i in 0..candidates.len() {
            let mut new_cur = cur.clone();
            new_cur.push(candidates[i]);
            _combination_sum(&candidates[i..], target, new_cur, result);
        }
    }
}

fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = vec![];
    _combination_sum(&candidates[0..], target, vec![], &mut result);
    result
}

#[test]
pub fn test_combination_sum() {
    assert_eq!(vec![vec![2, 2, 3], vec![7]], combination_sum(vec![2, 3, 6, 7], 7));
    assert_eq!(vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]], combination_sum(vec![2, 3, 5], 8));
    assert_eq!(vec![vec![8, 3], vec![7, 4], vec![4, 4, 3]], combination_sum(vec![8,7,4,3], 11));
}