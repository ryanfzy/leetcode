pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    fn _search(nums: &[i32], target: i32, result: &mut Vec<i32>, index: i32) {
        if nums.len() == 1 {
            if nums[0] == target {
                result.push(index);
            }
        } else {
            let p = nums.len() / 2;
            if target >= nums[0] && target <= nums[p-1] {
                _search(&nums[0..p], target, result, index);
            }
            if target >= nums[p] && target <= nums[nums.len()-1] {
                _search(&nums[p..], target, result, index + p as i32);
            }
        }
    }
    let mut result = vec![];
    if nums.len() > 0 {
        _search(&nums, target, &mut result, 0);
    }
    if result.len() > 0 {
        vec![*result.iter().min().unwrap(), *result.iter().max().unwrap()]
    } else {
        vec![-1, -1]
    }
}

#[test]
pub fn test_search_range() {
    assert_eq!(vec![3, 4], search_range(vec![5, 7, 7, 8, 8, 10], 8));
    assert_eq!(vec![-1, -1], search_range(vec![5, 7, 7, 8, 8, 10], 6));
    assert_eq!(vec![-1, -1], search_range(vec![], 0));
    assert_eq!(vec![0, 0], search_range(vec![8], 8));
    assert_eq!(vec![0, 1], search_range(vec![8, 8], 8));
}