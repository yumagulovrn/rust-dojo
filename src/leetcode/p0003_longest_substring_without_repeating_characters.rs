/**
 * [0003] Longest Substring Without Repeating Characters
 *
 * Given a string s, find the length of the longest substring without repeating characters.
 *
 * Example 1:
 * Input: s = "abcabcbb"
 * Output: 3
 * Explanation: The answer is "abc", with the length of 3.
 *
 * Example 2:
 * Input: s = "bbbbb"
 * Output: 1
 * Explanation: The answer is "b", with the length of 1.
 *
 * Example 3:
 * Input: s = "pwwkew"
 * Output: 3
 * Explanation: The answer is "wke", with the length of 3.
 * Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.
 *
 * Constraints:
 * * 0 <= s.length <= 5 * 10^4
 * * s consists of English letters, digits, symbols and spaces.
 */
use std::cmp::max;
use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut seen = HashMap::with_capacity(s.len());
        let mut max_len: i32 = 0;
        let mut start: i32 = 0;
        for (end, c) in s.chars().enumerate() {
            match seen.get(&c) {
                Some(idx) => {
                    start = max(start, idx + 1);
                }
                _ => (),
            }
            seen.insert(c, end as i32);
            max_len = max(max_len, end as i32 - start + 1);
        }
        max_len
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("abcabcbb", 3)]
    #[case("bbbbb", 1)]
    #[case("pwwkew", 3)]
    fn length_of_longest_substring(#[case] input: String, #[case] expected: i32) {
        assert_eq!(Solution::length_of_longest_substring(input), expected);
    }
}
