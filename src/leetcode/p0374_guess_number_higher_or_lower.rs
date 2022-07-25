/**
 * [0374] Guess Number Higher or Lower
 *
 * We are playing the Guess Game. The game is as follows:
 *
 * I pick a number from 1 to n. You have to guess which number I picked.
 *
 * Every time you guess wrong, I will tell you whether the number I picked is higher or lower than your guess.
 *
 * You call a pre-defined API int guess(int num), which returns three possible results:
 * * -1: Your guess is higher than the number I picked (i.e. num > pick).
 * * 1: Your guess is lower than the number I picked (i.e. num < pick).
 * * 0: your guess is equal to the number I picked (i.e. num == pick).
 * Return the number that I picked.
 *
 * Example 1:
 * Input: n = 10, pick = 6
 * Output: 6
 *
 * Example 2:
 * Input: n = 1, pick = 1
 * Output: 1
 *
 * Example 3:
 * Input: n = 2, pick = 1
 * Output: 1
 *
 * Constraints:
 * * 1 <= n <= 231 - 1
 * * 1 <= pick <= n
 */
use std::cmp::Ordering;
struct Solution {
    pick: i32,
}

impl Solution {
    fn guess(&self, num: i32) -> i32 {
        match self.pick.cmp(&num) {
            Ordering::Equal => 0,
            Ordering::Less => -1,
            Ordering::Greater => 1,
        }
    }

    fn guess_number(&self, n: i32) -> i32 {
        let (mut low, mut high) = (1, n);
        loop {
            let mid = low + (high - low) / 2;
            match self.guess(mid) {
                0 => break mid,
                -1 => high = mid - 1,
                _ => low = mid + 1,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(10, 6)]
    #[case(1, 1)]
    #[case(2, 1)]
    fn guess_number(#[case] n: i32, #[case] expected: i32) {
        let sln = Solution { pick: expected };
        assert_eq!(sln.guess_number(n), expected);
    }
}
