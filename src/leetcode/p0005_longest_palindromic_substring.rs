/**
 * [0005] Longest Palindromic Substring
 *
 * Given a string s, return the longest palindromic substring in s.
 *
 * Example 1:
 * Input: s = "babad"
 * Output: "bab"
 * Explanation: "aba" is also a valid answer.
 *
 * Example 2:
 * Input: s = "cbbd"
 * Output: "bb"
 *
 * Constraints:
 * * 1 <= s.length <= 1000
 * * s consist of only digits and English letters.
 */
use std::cmp;

struct Solution;

impl Solution {
    fn process_string(s: &str) -> String {
        let mut ss = String::with_capacity(s.len() * 2 + 3);
        ss.push('<');
        for ch in s.chars() {
            ss.push('|');
            ss.push(ch);
        }
        ss.push('|');
        ss.push('>');
        ss
    }

    pub fn longest_palindrome(s: String) -> String {
        if s.len() <= 1 {
            return s;
        }

        let ss: Vec<char> = Solution::process_string(&s).chars().collect();
        let length = ss.len();
        let mut sizes = vec![0; length];
        let (mut center, mut right, mut max_size, mut max_size_center) = (0, 0, 0, 0);
        let mut i = 1;

        while i < length - 1 {
            if i < right {
                sizes[i] = cmp::min(sizes[2 * center - i], right - i)
            }

            while i + sizes[i] + 1 < length
                && i - sizes[i] - 1 > 0
                && ss[i + sizes[i] + 1] == ss[i - sizes[i] - 1]
            {
                sizes[i] += 1;
            }

            if sizes[i] > max_size {
                max_size = sizes[i];
                max_size_center = i;
            }

            if i + sizes[i] > right {
                center = i;
                right = i + sizes[i];
            }

            i += 1;
        }

        let start = (max_size_center - max_size) / 2;
        let end = start + max_size - 1;

        return s[start..=end].chars().collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::longest_palindrome("babad".to_string()), "bab");
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::longest_palindrome("cbbd".to_string()), "bb");
    }
}
