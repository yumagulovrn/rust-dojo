/**
 * [0010] Regular Expression Matching
 *
 * Given an input string s and a pattern p, implement regular expression matching with support for '.' and '*' where:
 * * '.' Matches any single character.​​​​
 * * '*' Matches zero or more of the preceding element.
 * The matching should cover the entire input string (not partial).
 *
 * Example 1:
 * Input: s = "aa", p = "a"
 * Output: false
 * Explanation: "a" does not match the entire string "aa".
 *
 * Example 2:
 * Input: s = "aa", p = "a*"
 * Output: true
 * Explanation: '*' means zero or more of the preceding element, 'a'. Therefore, by repeating 'a' once, it becomes "aa".
 *
 * Example 3:
 * Input: s = "ab", p = ".*"
 * Output: true
 * Explanation: ".*" means "zero or more (*) of any character (.)".
 *
 * Constraints:
 * * 1 <= s.length <= 20
 * * 1 <= p.length <= 30
 * * s contains only lowercase English letters.
 * * p contains only lowercase English letters, '.', and '*'.
 * * It is guaranteed for each appearance of the character '*', there will be a previous valid character to match.
 */
struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let c_len = p.len() + 1;
        let length = (s.len() + 1) * c_len;
        let s: Vec<char> = s.chars().collect();
        let p: Vec<char> = p.chars().collect();
        let mut dp = vec![false; length];

        dp[length - 1] = true;

        for i in (0..=s.len()).rev() {
            for j in (0..p.len()).rev() {
                let first_match = i < s.len() && (p[j] == s[i] || p[j] == '.');

                if j + 1 < p.len() && p[j + 1] == '*' {
                    dp[i * c_len + j] =
                        dp[i * c_len + j + 2] || first_match && dp[(i + 1) * c_len + j];
                } else {
                    dp[i * c_len + j] = first_match && dp[(i + 1) * c_len + j + 1];
                }
            }
        }
        dp[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::is_match("aa".to_string(), "a".to_string()), false);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::is_match("aa".to_string(), "a*".to_string()), true);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::is_match("ab".to_string(), ".*".to_string()), true);
    }

    #[test]
    fn test_4() {
        assert_eq!(
            Solution::is_match("aaa".to_string(), "a*a".to_string()),
            true
        );
    }
}
