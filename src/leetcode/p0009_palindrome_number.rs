/**
 * [0009] Palindrome Number
 *
 * Given an integer x, return true if x is palindrome integer.
 * An integer is a palindrome when it reads the same backward as forward.
 * * For example, 121 is a palindrome while 123 is not.
 *
 * Example 1:
 * Input: x = 121
 * Output: true
 * Explanation: 121 reads as 121 from left to right and from right to left.
 *
 * Example 2:
 * Input: x = -121
 * Output: false
 * Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.
 *
 * Example 3:
 * Input: x = 10
 * Output: false
 * Explanation: Reads 01 from right to left. Therefore it is not a palindrome.
 *  
 *
 * Constraints:
 * * -2^31 <= x <= 2^31 - 1
 */
struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 || (x % 10 == 0 && x != 0) {
            false
        } else {
            let (mut head, mut tail) = (x, 0);
            while head > tail {
                tail = tail * 10 + head % 10;
                head /= 10;
            }
            head == tail || head == tail / 10
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(121, true)]
    #[case(-121, false)]
    #[case(10, false)]
    #[case(i32::MAX, false)]
    #[case(i32::MIN, false)]
    fn is_palindrome(#[case] input: i32, #[case] expected: bool) {
        assert_eq!(Solution::is_palindrome(input), expected);
    }
}
