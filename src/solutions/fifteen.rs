pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = vec![];
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
                    if s > 0 {
                        r -= 1;
                    } else if s < 0 {
                        l += 1;
                    } else {
                        result.push(vec![a, b, c]);
                        loop {
                            l += 1;
                            if l >= r || n[l] != n[l-1] {
                                break;
                            }
                        }
                        loop {
                            r -= 1;
                            if r <= l || n[r] != n[r+1] {
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
    result
}

#[test]
pub fn test_three_sum() {
    //assert_eq!(vec![vec![-1,0,1], vec![-1,-1,2]], three_sum(vec![-1,0,1,2,-1,-4]));
    assert_eq!(vec![vec![0,0,0]], three_sum(vec![0,0,0,0]));
    //assert_eq!(vec![vec![0]], three_sum(vec![1,2,-2,-1]));
}