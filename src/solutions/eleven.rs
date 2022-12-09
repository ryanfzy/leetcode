
// O(n^2)
pub fn max_area(height: Vec<i32>) -> i32 {
    use std::cmp::min;
    use std::cmp::max;
    let mut r = 0;
    for i in 1..height.len() {
        let mut e = i;
        if r > height[i] {
            let d = (r / max(height[i], 1)) as usize;
            if e > d {
                e = e - d
            }
        }
        for j in 0..e {
            let m = min(height[i], height[j]) * (i - j) as i32;
            r = max(m, r);
        }
    }
    r
}

// O(n)
pub fn max_area2(height: Vec<i32>) -> i32 {
    use std::cmp::min;
    use std::cmp::max;
    let mut r = 0;
    let mut it = height.iter().enumerate();
    let mut p1 = it.next();
    let mut p2 = it.next_back();
    while let (Some((i, left)), Some((j, right))) = (p1, p2) {
        r = max(r, min(left, right) * (j - i) as i32);
        if left < right {
            p1 = it.next();
        } else {
            p2 = it.next_back();
        }
    }
    r
}

#[test]
pub fn test_max_area() {
    assert_eq!(49, max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]));
    assert_eq!(1, max_area(vec![1, 1]));
    assert_eq!(1, max_area(vec![1, 2]));
}

#[test]
pub fn test_max_area2() {
    assert_eq!(49, max_area2(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]));
    assert_eq!(1, max_area2(vec![1, 1]));
    assert_eq!(1, max_area2(vec![1, 2]));
}