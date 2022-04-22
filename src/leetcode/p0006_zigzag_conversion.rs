/**
 * [0006] Zigzag Conversion
 *
 * The string "PAYPALISHIRING" is written in a zigzag pattern on a given number of rows like this: (you may want to display this pattern in a fixed font for better legibility)
 *
 * P   A   H   N
 * A P L S I I G
 * Y   I   R
 *
 * And then read line by line: "PAHNAPLSIIGYIR"
 * Write the code that will take a string and make this conversion given a number of rows:
 * string convert(string s, int numRows);
 *
 * Example 1:
 * Input: s = "PAYPALISHIRING", numRows = 3
 * Output: "PAHNAPLSIIGYIR"
 * Explanation:
 * P   A   H   N
 * A P L S I I G
 * Y   I   R
 *
 * Example 2:
 * Input: s = "PAYPALISHIRING", numRows = 4
 * Output: "PINALSIGYAHRPI"
 * Explanation:
 * P     I    N
 * A   L S  I G
 * Y A   H R
 * P     I
 *
 * Example 3:
 * Input: s = "A", numRows = 1
 * Output: "A"
 *
 * Constraints:
 * * 1 <= s.length <= 1000
 * * s consists of English letters (lower-case and upper-case), ',' and '.'.
 * * 1 <= numRows <= 1000
 */
struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let mut zigzags: Vec<_> = (0..num_rows)
            .chain((1..num_rows - 1).rev())
            .cycle()
            .zip(s.chars())
            .collect();
        zigzags.sort_by_key(|&(row, _)| row);
        zigzags.into_iter().map(|(_, c)| c).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("PAYPALISHIRING", 3, "PAHNAPLSIIGYIR")]
    #[case("PAYPALISHIRING", 4, "PINALSIGYAHRPI")]
    #[case("A", 1, "A")]
    fn convert(#[case] s: String, #[case] num_rows: i32, #[case] expected: String) {
        assert_eq!(Solution::convert(s, num_rows), expected);
    }
}
