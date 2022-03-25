struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() < 3 {
            return vec![];
        }

        let mut res: Vec<Vec<i32>> = vec![];
        let mut nums = nums;
        nums.sort_unstable();

        let mut i = 0;

        while i < nums.len() - 2 {
            if nums[i] > 0 {
                break;
            }
            while i < nums.len() - 2 && i > 0 && nums[i] == nums[i - 1] {
                i += 1;
            }

            let mut j = i + 1;
            let mut k = nums.len() - 1;

            while j < k {
                let sum = nums[i] + nums[j] + nums[k];
                if sum == 0 {
                    res.push(vec![nums[i], nums[j], nums[k]]);
                    j += 1;
                    k -= 1;
                    while j < k && nums[j] == nums[j - 1] {
                        j += 1;
                    }
                    while k > j && nums[k] == nums[k + 1] {
                        k -= 1;
                    }
                } else if sum < 0 {
                    // < 0 so need to increase j
                    j += 1;
                    while j < k && nums[j] == nums[j - 1] {
                        j += 1;
                    }
                } else {
                    // > 0, so need to decrease k
                    k -= 1;
                    while k > j && nums[k] == nums[k + 1] {
                        k -= 1;
                    }
                }
            }

            i += 1;
        }

        res
    }
}

#[test]
fn test1() {
    let nums = vec![-1, 0, 1, 2, -1, -4];
    assert_eq!(
        Solution::three_sum(nums),
        vec![vec![-1, -1, 2], vec![-1, 0, 1]]
    );
}
