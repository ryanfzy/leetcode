pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() {
        for j in (i+1)..nums.len() {
            if (nums[i] + nums[j]) == target {
                return vec![i as i32, j as i32];
            }
        }
    }
    vec![]
}

#[test]
fn test_two_sum() {
    let nums = vec![2, 7, 11, 15];
    let target: i32 = 9;
    let output = two_sum(nums, target);
    assert_eq!(output[0], 0);
    assert_eq!(output[1], 1);
}