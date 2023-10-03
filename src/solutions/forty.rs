fn _combination_sum(candidates: &[i32], target: i32, cur: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
    let sum: i32 = cur.iter().sum();
    if sum == target {
        result.push(cur.clone());
    } else if sum < target {
        for i in 0..candidates.len() {
            if i > 0 && candidates[i] == candidates[i-1] {
                continue;
            } else {
                cur.push(candidates[i]);
                _combination_sum(&candidates[i+1..], target, cur, result);
                cur.pop();
            }
        }
    }
}

pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = vec![];
    let mut cur: Vec<i32> = vec![];
    let mut sorted = candidates.clone();
    sorted.sort();
    _combination_sum(&sorted[0..], target, &mut cur, &mut result);
    result
}

#[test]
pub fn test_combination_sum2() {
    assert_eq!(vec![vec![1, 1, 6], vec![1, 2, 5],  vec![1, 7], vec![2, 6]], combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8));
    assert_eq!(vec![vec![1, 2, 2], vec![5]], combination_sum2(vec![2, 5, 2, 1, 2], 5));
}