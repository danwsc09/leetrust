struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        // let mut result = 0;
        // let mut abs_x = i32::abs(x);
        // while abs_x > 0 {
        //     result = result * 10 + abs_x % 10;
        //     abs_x = abs_x / 10;
        // }

        // if x < 0 {
        //     return -result;
        // }
        // result
        // let str = if x > 0 {
        //     i32::to_string(&x)
        // } else {
        //     i32::to_string(&(-x))
        // };
    }
}

#[test]
fn test_l7_1() {
    assert_eq!(Solution::reverse(123), 321);
}

#[test]
fn test_l7_2() {
    assert_eq!(Solution::reverse(-123), -321);
}

#[test]
fn test_l7_3() {
    assert_eq!(Solution::reverse(120), 21);
}
