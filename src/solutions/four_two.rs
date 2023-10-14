pub fn trap(height: Vec<i32>) -> i32 {
    use std::cmp::min;
    let mut v: Vec<(i32, usize)> = vec![];
    let mut cap = 0;
    for i in 0..height.len() {
        if let Some(h) = v.pop() {
            if h.0 <= height[i] {
                v.push(h);
                let mut prev = 0;
                while let Some(h) = v.pop() {
                    let min_h = min(height[i], h.0);
                    cap += (i - h.1 - 1) as i32 * (min_h - prev);
                    prev = h.0;
                    if h.0 > height[i] {
                        v.push(h);
                        break;
                    }
                }
            } else {
                v.push(h);
            }
        }
        v.push((height[i], i)); 
    }
    cap as i32
}

fn trap2(height: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = height.len() - 1;
    let mut h = 0;
    let mut cap = 0;
    while left < right {
        h = height[left].min(height[right]).max(h);
        if height[left] <= height[right] {
            cap += h - height[left];
            left += 1;
        } else {
            cap += h - height[right];
            right -= 1;
        }
    }
    cap
}

#[test]
pub fn test_trap() {
    assert_eq!(6, trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]));
    assert_eq!(9, trap(vec![4, 2, 0, 3, 2, 5]));
    assert_eq!(83, trap(vec![6,4,2,0,3,2,0,3,1,4,5,3,2,7,5,3,0,1,2,1,3,4,6,8,1,3]));
}

#[test]
pub fn test_trap2() {
    assert_eq!(6, trap2(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]));
    assert_eq!(9, trap2(vec![4, 2, 0, 3, 2, 5]));
    assert_eq!(83, trap2(vec![6,4,2,0,3,2,0,3,1,4,5,3,2,7,5,3,0,1,2,1,3,4,6,8,1,3]));
}