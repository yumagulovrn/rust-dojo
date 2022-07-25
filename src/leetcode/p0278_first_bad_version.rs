/**
 * [0278] First Bad Version
 *
 * You are a product manager and currently leading a team to develop a new product. Unfortunately, the latest version of your product fails the quality check. Since each version is developed based on the previous version, all the versions after a bad version are also bad.
 *
 * Suppose you have n versions [1, 2, ..., n] and you want to find out the first bad one, which causes all the following ones to be bad.
 *
 * You are given an API bool isBadVersion(version) which returns whether version is bad. Implement a function to find the first bad version. You should minimize the number of calls to the API.
 *
 * Example 1:
 * Input: n = 5, bad = 4
 * Output: 4
 * Explanation:
 * call isBadVersion(3) -> false
 * call isBadVersion(5) -> true
 * call isBadVersion(4) -> true
 * Then 4 is the first bad version.
 *
 * Example 2:
 * Input: n = 1, bad = 1
 * Output: 1
 *
 * Constraints:
 * * 1 <= bad <= n <= 231 - 1
 */
struct Solution {
    bad_version: i32,
}

impl Solution {
    fn is_bad_version(&self, version: i32) -> bool {
        version >= self.bad_version
    }

    pub fn first_bad_version(&self, n: i32) -> i32 {
        let (mut low, mut high) = (0, n);
        while low <= high {
            let mid = low + (high - low) / 2;
            match self.is_bad_version(mid) {
                false => low = mid + 1,
                true => high = mid - 1,
            }
        }
        low
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(5, 4)]
    #[case(1, 1)]
    fn search(#[case] input: i32, #[case] expected: i32) {
        let sln = Solution {
            bad_version: expected,
        };
        assert_eq!(sln.first_bad_version(input), expected);
    }
}
