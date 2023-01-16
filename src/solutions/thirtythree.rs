pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    fn _search(nums: &[i32], target: i32, index: i32) -> i32 {
        if nums.len() == 1 {
            if nums[0] == target {
                return index;
            } else {
                return -1;
            }
        }
        let p = nums.len() / 2;
        if target == nums[p] {
            return index + p as i32;
        } else if target == nums[0] {
            return index;
        } else if nums[0] > nums[p-1] && (target >= nums[0] || target <= nums[p-1]) {
            return _search(&nums[..p], target, index);
        } else if target >= nums[0] && target <= nums[p-1] {
            return _search(&nums[..p], target, index);
        } else {
            return _search(&nums[p..], target, index + p as i32);
        }
    }
    _search(&nums, target, 0)
}

#[test]
pub fn test_search() {
    assert_eq!(3, search(vec![1, 2, 3, 4, 5, 6, 7], 4));
    assert_eq!(-1, search(vec![1, 2, 3, 5, 6, 7], 4));
    assert_eq!(4, search(vec![4, 5, 6, 7, 0, 1, 2], 0));
    assert_eq!(0, search(vec![1, 3], 1));
    assert_eq!(2, search(vec![3, 5, 1], 1));
    assert_eq!(0, search(vec![4, 5, 6, 7, 0, 1, 2], 4));
    assert_eq!(0, search(vec![6, 7, 1, 2, 3, 4, 5], 6));
    assert_eq!(1, search(vec![8, 9, 2, 3, 4], 9));
}