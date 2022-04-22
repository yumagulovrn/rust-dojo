/**
 * [0008] String to Integer (atoi)
 *
 * Implement the myAtoi(string s) function, which converts a string to a 32-bit signed integer (similar to C/C++'s atoi function).
 *
 * The algorithm for myAtoi(string s) is as follows:
 * 1. Read in and ignore any leading whitespace.
 * 2. Check if the next character (if not already at the end of the string) is '-' or '+'. Read this character in if it is either. This determines if the final result is negative or positive respectively. Assume the result is positive if neither is present.
 * 3. Read in next the characters until the next non-digit character or the end of the input is reached. The rest of the string is ignored.
 * 4. Convert these digits into an integer (i.e. "123" -> 123, "0032" -> 32). If no digits were read, then the integer is 0. Change the sign as necessary (from step 2).
 * 5. If the integer is out of the 32-bit signed integer range [-231, 231 - 1], then clamp the integer so that it remains in the range. Specifically, integers less than -231 should be clamped to -231, and integers greater than 231 - 1 should be clamped to 231 - 1.
 * 6. Return the integer as the final result.
 *
 * Note:
 * * Only the space character ' ' is considered a whitespace character.
 * * Do not ignore any characters other than the leading whitespace or the rest of the string after the digits.
 *
 * Example 1:
 * Input: s = "42"
 * Output: 42
 * Explanation: The underlined characters are what is read in, the caret is the current reader position.
 * Step 1: "42" (no characters read because there is no leading whitespace)
 *          ^
 * Step 2: "42" (no characters read because there is neither a '-' nor '+')
 *          ^
 * Step 3: "42" ("42" is read in)
 *            ^
 * The parsed integer is 42.
 * Since 42 is in the range [-231, 231 - 1], the final result is 42.
 *
 * Example 2:
 * Input: s = "   -42"
 * Output: -42
 * Explanation:
 * Step 1: "   -42" (leading whitespace is read and ignored)
 *             ^
 * Step 2: "   -42" ('-' is read, so the result should be negative)
 *              ^
 * Step 3: "   -42" ("42" is read in)
 *                ^
 * The parsed integer is -42.
 * Since -42 is in the range [-231, 231 - 1], the final result is -42.
 *
 * Example 3:
 * Input: s = "4193 with words"
 * Output: 4193
 * Explanation:
 * Step 1: "4193 with words" (no characters read because there is no leading whitespace)
 *          ^
 * Step 2: "4193 with words" (no characters read because there is neither a '-' nor '+')
 *          ^
 * Step 3: "4193 with words" ("4193" is read in; reading stops because the next character is a non-digit)
 *              ^
 * The parsed integer is 4193.
 * Since 4193 is in the range [-2^31, 2^31 - 1], the final result is 4193.
 *  
 *
 * Constraints:
 * * 0 <= s.length <= 200
 * * s consists of English letters (lower-case and upper-case), digits (0-9), ' ', '+', '-', and '.'.
 */
struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut res = 0i32;
        let mut is_started = false;
        let mut is_negative = false;
        for c in s.chars() {
            match c {
                ' ' if !is_started => continue,
                '+' if !is_started => is_started = true,
                '-' if !is_started & !is_negative => {
                    is_started = true;
                    is_negative = true;
                }
                '0'..='9' => {
                    is_started = true;

                    res = match res
                        .checked_mul(10)
                        .and_then(|res| res.checked_add(c as i32 - '0' as i32))
                    {
                        Some(v) => v,
                        None => return if is_negative { i32::MIN } else { i32::MAX },
                    };
                }
                _ => break,
            }
        }

        if is_negative {
            -res
        } else {
            res
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("42", 42)]
    #[case("   -42", -42)]
    #[case("4193 with words", 4193)]
    #[case("-2147483648", i32::MIN)]
    #[case("2147483648", i32::MAX)]
    fn my_atoi(#[case] input: String, #[case] expected: i32) {
        assert_eq!(Solution::my_atoi(input), expected);
    }
}
