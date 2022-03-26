struct Solution;

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let n = nums.len();
        let mut min_diff = i32::MAX;

        for i in 0..n - 2 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let mut j = i + 1;
            let mut k = n - 1;

            while j < k {
                let sum = nums[i] + nums[j] + nums[k];
                if sum == target {
                    return sum;
                }
                if i32::abs(sum - target) < i32::abs(min_diff) {
                    min_diff = sum - target;
                }
                if sum < target {
                    j += 1;
                } else {
                    k -= 1;
                }
            }
        }
        min_diff + target
    }
}

#[test]
fn test_l16_1() {
    let nums = vec![-1, 2, 1, -4];
    let target = 1;
    assert_eq!(Solution::three_sum_closest(nums, target), 2);
}

#[test]
fn test_l16_2() {
    let nums = vec![0, 0, 0];
    let target = 1;
    assert_eq!(Solution::three_sum_closest(nums, target), 0);
}
