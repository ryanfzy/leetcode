pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    if nums.len() < 4 {
        return vec![];
    }
    let mut n = nums.clone();
    n.sort();
    let mut result = vec![];
    fn three_sum(n: &[i32], target: i32) -> Vec<Vec<i32>> {
        let mut result = vec![];
        for i in 0..n.len()-2 {
            if i > 0 && n[i] == n[i-1] {
                continue;
            }
            let a = n[i];
            let mut l = i+1;
            let mut r = n.len()-1;
            while l < r {
                let b = n[l];
                let c = n[r];
                if let Some(t) = a.checked_add(b) {
                    if let Some(s) = t.checked_add(c) {
                        if s > target {
                            r -= 1;
                        } else if s < target {
                            l += 1;
                        } else {
                            result.push(vec![a, b, c]);
                            loop {
                                r -= 1;
                                if r <= l || n[r] != n[r+1] {
                                    break;
                                }
                            }
                            loop {
                                l += 1;
                                if l >= r || n[l] != n[l-1] {
                                    break;
                                }
                            }
                        }
                    } else {
                        r -= 1;
                    }
                } else {
                    r -= 1;
                }
            }
        }
        result
    }
    for i in 0..n.len()-3 {
        if i > 0 && n[i-1] == n[i] {
            continue;
        }
        let x = n[i];
        for r in three_sum(&n[i+1..], target-x) {
            let mut v = vec![x];
            v.extend(&r);
            result.push(v);
        }
    }
    result
}

#[test]
pub fn test_four_sum() {
    assert_eq!(vec![vec![2, 2, 2, 2]], four_sum(vec![2, 2, 2, 2, 2], 8));
    assert_eq!(vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]], four_sum(vec![1,0,-1,0,-2,2], 0));
    assert_eq!(vec![] as Vec<Vec<i32>>, four_sum(vec![1000000000,1000000000,1000000000,1000000000], 0));
}