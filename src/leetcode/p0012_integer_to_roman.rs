/**
 * [0012] Integer to Roman
 *
 * Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M.
 *
 * Symbol       Value
 * I             1
 * V             5
 * X             10
 * L             50
 * C             100
 * D             500
 * M             1000
 *
 * For example, 2 is written as II in Roman numeral, just two one's added together. 12 is written as XII, which is simply X + II. The number 27 is written as XXVII, which is XX + V + II.
 *
 * Roman numerals are usually written largest to smallest from left to right. However, the numeral for four is not IIII. Instead, the number four is written as IV. Because the one is before the five we subtract it making four. The same principle applies to the number nine, which is written as IX. There are six instances where subtraction is used:
 *
 * * I can be placed before V (5) and X (10) to make 4 and 9.
 * * X can be placed before L (50) and C (100) to make 40 and 90.
 * * C can be placed before D (500) and M (1000) to make 400 and 900.
 *
 * Given an integer, convert it to a roman numeral.
 *
 * Example 1:
 * Input: num = 3
 * Output: "III"
 * Explanation: 3 is represented as 3 ones.
 *
 * Example 2:
 * Input: num = 58
 * Output: "LVIII"
 * Explanation: L = 50, V = 5, III = 3.
 *
 * Example 3:
 * Input: num = 1994
 * Output: "MCMXCIV"
 * Explanation: M = 1000, CM = 900, XC = 90 and IV = 4.
 */
struct Solution;

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let ones = ['I', 'X', 'C', 'M'];
        let fives = ['V', 'L', 'D'];

        let mut digits = Vec::new();
        let mut num = num;

        while num != 0 {
            digits.push(num % 10);
            num /= 10
        }

        digits
            .into_iter()
            .enumerate()
            .rev()
            .map(|(i, digit)| match digit {
                1 => vec![ones[i]],
                2 => vec![ones[i]; 2],
                3 => vec![ones[i]; 3],
                4 => vec![ones[i], fives[i]],
                5 => vec![fives[i]],
                6 => vec![fives[i], ones[i]],
                7 => vec![fives[i], ones[i], ones[i]],
                8 => vec![fives[i], ones[i], ones[i], ones[i]],
                9 => vec![ones[i], ones[i + 1]],
                _ => Vec::new(),
            })
            .flatten()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(3, "III")]
    #[case(58, "LVIII")]
    #[case(1994, "MCMXCIV")]
    fn int_to_roman(#[case] input: i32, #[case] expected: String) {
        assert_eq!(Solution::int_to_roman(input), expected);
    }
}
