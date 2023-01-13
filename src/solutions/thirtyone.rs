
// search from the front
pub fn next_permutation(nums: &mut Vec<i32>) {
    pub fn _next(nums: &mut Vec<i32>, start: usize) -> i32 {
        let len = nums.len();
        for i in start..nums.len()-1 {
            if nums[i] >= nums[i+1] {
                match _next(nums, i+1) {
                    1 => {
                        if i >= 1 {
                            for j in (i..nums.len()).rev() {
                                if nums[j] > nums[i-1] {
                                    nums.swap(i-1, j);
                                    nums[i..].sort();
                                    return 0;
                                }
                            }
                        }
                        return 1;
                    },
                    2 => {
                        nums.swap(len-2, len-1);
                        return 0;
                    },
                    r => {
                        return r;
                    }
                }
            }
        }
        if nums[len-1] > nums[len-2] {
            return 2;
        } else {
            return 1;
        }
    }
    let len = nums.len();
    if len > 1 {
        match _next(nums, 0) {
            1 => {
                nums.sort();
            },
            2 => {
                nums.swap(len-2, len-1);
            },
            _ => ()
        }
    }
}

// searching from the end, same efficiency but but code is cleaner
pub fn next_permutation2(nums: &mut Vec<i32>) {
    let mut i = nums.len() - 1;
    while i > 0 && nums[i] <= nums[i-1] {
        i -= 1;
    }
    if i > 0 {
        let mut j = nums.len() - 1;
        while j > 0  {
            if nums[j] > nums[i-1] {
                nums.swap(j, i-1);
                break;
            }
            j -= 1;
        }
        nums[i..].sort();
    } else {
        nums.sort();
    }
}

#[test]
pub fn test_next_permutation() {
    let mut test1 = vec![1, 2, 3, 4, 5];
    next_permutation(&mut test1);
    assert_eq!(vec![1, 2, 3, 5, 4], test1);
    next_permutation(&mut test1);
    assert_eq!(vec![1, 2, 4, 3, 5], test1);
    next_permutation(&mut test1);
    assert_eq!(vec![1, 2, 4, 5, 3], test1);
    next_permutation(&mut test1);
    assert_eq!(vec![1, 2, 5, 3, 4], test1);
    next_permutation(&mut test1);
    assert_eq!(vec![1, 2, 5, 4, 3], test1);
    next_permutation(&mut test1);
    assert_eq!(vec![1, 3, 2, 4, 5], test1);

    let mut test2 = vec![5, 4, 3, 2, 1];
    next_permutation(&mut test2);
    assert_eq!(vec![1, 2, 3, 4, 5], test2);

    let mut test3 = vec![5,4,7,5,3,2];
    next_permutation(&mut test3);
    assert_eq!(vec![5,5,2,3,4,7], test3);
}

#[test]
pub fn test_next_permutation2() {
    let mut test1 = vec![1, 2, 3, 4, 5];
    next_permutation2(&mut test1);
    assert_eq!(vec![1, 2, 3, 5, 4], test1);
    next_permutation2(&mut test1);
    assert_eq!(vec![1, 2, 4, 3, 5], test1);
    next_permutation2(&mut test1);
    assert_eq!(vec![1, 2, 4, 5, 3], test1);
    next_permutation2(&mut test1);
    assert_eq!(vec![1, 2, 5, 3, 4], test1);
    next_permutation2(&mut test1);
    assert_eq!(vec![1, 2, 5, 4, 3], test1);
    next_permutation2(&mut test1);
    assert_eq!(vec![1, 3, 2, 4, 5], test1);

    let mut test2 = vec![5, 4, 3, 2, 1];
    next_permutation2(&mut test2);
    assert_eq!(vec![1, 2, 3, 4, 5], test2);

    let mut test3 = vec![5,4,7,5,3,2];
    next_permutation2(&mut test3);
    assert_eq!(vec![5,5,2,3,4,7], test3);

    let mut test4 = vec![5, 1, 1];
    next_permutation2(&mut test4);
    assert_eq!(vec![1, 1, 5], test4);
}