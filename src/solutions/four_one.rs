fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
    for i in 0..nums.len() {
        let mut tmp = nums[i];
        loop {
            if tmp == i as i32 + 1 {
                break;
            } else if tmp <= 0 || tmp > nums.len() as i32 {
                break;
            } else {
                let tmp2 = nums[tmp as usize - 1];
                if tmp == tmp2 {
                    break;
                }
                nums[tmp as usize - 1] = tmp;
                tmp = tmp2;
            }
        }
        nums[i] = tmp;
    }
    for i in 0..nums.len() {
        if nums[i] != i as i32 + 1 {
            return i as i32 + 1;
        }
    }
    nums.len() as i32 + 1
}

#[test]
pub fn test_first_missing_positive() {
    assert_eq!(3, first_missing_positive(vec![1, 2, 0]));
    assert_eq!(2, first_missing_positive(vec![3, 4, -1, 1]));
    assert_eq!(1, first_missing_positive(vec![7, 8, 9, 11, 12]));
    assert_eq!(3, first_missing_positive(vec![2, 1]));
    assert_eq!(2, first_missing_positive(vec![1, 1]));
}
