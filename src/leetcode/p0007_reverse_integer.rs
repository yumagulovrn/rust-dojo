/**
 * [0007] Reverse Integer
 *
 * Given a signed 32-bit integer x, return x with its digits reversed. If reversing x causes the value to go outside the signed 32-bit integer range [-231, 231 - 1], then return 0.
 * Assume the environment does not allow you to store 64-bit integers (signed or unsigned).
 *
 * Example 1:
 * Input: x = 123
 * Output: 321
 *
 * Example 2:
 * Input: x = -123
 * Output: -321
 *
 * Example 3:
 * Input: x = 120
 * Output: 21
 *
 * Constraints:
 * * -2^31 <= x <= 2^31 - 1
 */
struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut res: i32 = 0;
        let mut cur: i32 = x;
        while cur != 0 {
            match res.checked_mul(10) {
                None => return 0,
                Some(tmp) => match tmp.checked_add(cur % 10) {
                    None => return 0,
                    Some(fine) => {
                        res = fine;
                    }
                },
            }
            cur /= 10;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(123, 321)]
    #[case(-123, -321)]
    #[case(120, 21)]
    #[case(2147483647, 0)]
    fn reverse(#[case] input: i32, #[case] expected: i32) {
        assert_eq!(Solution::reverse(input), expected);
    }
}
