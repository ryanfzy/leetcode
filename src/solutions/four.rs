pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let total_length = nums1.len() + nums2.len();
    let i = total_length / 2;
    let j = match total_length % 2 {
        0 => i - 1,
        _ => i
    };
    let mut n = 0;
    let mut m = 0;
    let mut cur = 0;
    let mut t = 0;
    while cur <= i {
        let cur_val;
        if n >= nums1.len() {
            cur_val = nums2[m];
            m += 1;
        } else if m >= nums2.len() {
            cur_val = nums1[n];
            n += 1;
        } else if nums1[n] <= nums2[m] {
            cur_val = nums1[n];
            n += 1;
        } else {
            cur_val = nums2[m];
            m += 1;
        }
        if cur == j {
            t += cur_val;
        }
        if cur == i {
            t += cur_val;
        }
        
        cur += 1;
    }
    t as f64 / 2.0
}

#[test]
pub fn test_find_median_sorted_arrays() {
    let nums1 = vec![1, 3];
    let nums2 = vec![2];
    assert_eq!(find_median_sorted_arrays(nums1, nums2), 2.0);
}

#[test]
pub fn test_find_median_sorted_arrays2() {
    let nums1 = vec![1, 2];
    let nums2 = vec![3, 4];
    assert_eq!(find_median_sorted_arrays(nums1, nums2), 2.5);
}