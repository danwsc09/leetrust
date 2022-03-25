struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hm: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
        for i in 0..nums.len() {
            if let Some(j) = hm.get(&(target - nums[i])) {
                return vec![i as i32, *j as i32];
            }
            hm.insert(nums[i], i as i32);
        }

        vec![]
    }
    // pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    //     let mut hm: HashMap<i32, i32> = HashMap::new();
    //     for (i, num) in nums.iter().enumerate() {
    //         if let Some(&j) = hm.get(&(target - num)) {
    //             return vec![j, i as i32];
    //         }
    //         hm.insert(*num, i as i32);
    //     }
    //     vec![]
    // }
}

#[test]
fn test() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    assert_eq!(Solution::two_sum(nums, target), [0, 1]);
}

#[test]
fn test1() {
    let nums = vec![3, 2, 4];
    let target = 6;
    assert_eq!(Solution::two_sum(nums, target), [1, 2]);
}
