/**
 * [0014] Longest Common Prefix
 *
 * Write a function to find the longest common prefix string amongst an array of strings.
 *
 * If there is no common prefix, return an empty string "".
 *
 * Example 1:
 * Input: strs = ["flower","flow","flight"]
 * Output: "fl"
 *
 * Example 2:
 * Input: strs = ["dog","racecar","car"]
 * Output: ""
 * Explanation: There is no common prefix among the input strings.
 *
 * Constraints:
 * * 1 <= strs.length <= 200
 * * 0 <= strs[i].length <= 200
 * * strs[i] consists of only lowercase English letters.
 */
struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        match strs.is_empty() {
            true => "".to_string(),
            _ => strs.iter().skip(1).fold(strs[0].clone(), |acc, x| {
                acc.chars()
                    .zip(x.chars())
                    .take_while(|(x, y)| x == y)
                    .map(|(x, _)| x)
                    .collect()
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec!["flower".to_string(), "flow".to_string(), "flight".to_string()], "fl".to_string())]
    #[case(vec!["dog".to_string(), "racecar".to_string(), "car".to_string()], "".to_string())]
    #[case(vec!["shitty".to_string(), "shitpost".to_string(), "shite".to_string(), "shitttte".to_string()], "shit".to_string())]
    fn longest_common_prefix(#[case] input: Vec<String>, #[case] expected: String) {
        assert_eq!(Solution::longest_common_prefix(input), expected);
    }
}
