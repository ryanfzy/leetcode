pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
    let mut result = i32::MAX;
    let mut n = nums.clone();
    n.sort();
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
                    if (s - target).abs() < (result - target).abs() {
                        result = s;
                    }
                    if s > target {
                        loop {
                            r -= 1;
                            if r <= l || n[r] != n[r+1] {
                                break;
                            }
                        }
                    } else if s < target {
                        loop {
                            l += 1;
                            if l >= r || n[l] != n[l-1] {
                                break;
                            }
                        }
                    } else {
                        break;
                    }
                }
            }
        }
    }
    result
}

#[test]
pub fn test_three_sum_closest() {
    assert_eq!(2, three_sum_closest(vec![-1, 2, 1, -4], 1));
    assert_eq!(0, three_sum_closest(vec![0, 0, 0], 1));
}