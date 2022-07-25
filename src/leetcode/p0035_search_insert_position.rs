/**
 * [0035] Search Insert Position
 *
 * Given a sorted array of distinct integers and a target value, return the index if the target is found. If not, return the index where it would be if it were inserted in order.
 *
 * You must write an algorithm with O(log n) runtime complexity.
 *
 * Example 1:
 * Input: nums = [1,3,5,6], target = 5
 * Output: 2
 *
 * Example 2:
 * Input: nums = [1,3,5,6], target = 2
 * Output: 1
 *
 * Example 3:
 * Input: nums = [1,3,5,6], target = 7
 * Output: 4
 *
 * Constraints:
 * * 1 <= nums.length <= 104
 * * -104 <= nums[i] <= 104
 * * nums contains distinct values sorted in ascending order.
 * * -104 <= target <= 104
 */
use std::cmp::Ordering;
struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let (mut low, mut high) = (0, nums.len());
        while low < high {
            let mid = low + (high - low) / 2;
            match nums[mid].cmp(&target) {
                Ordering::Equal => return mid as i32,
                Ordering::Less => low = mid + 1,
                Ordering::Greater => high = mid,
            }
        }
        low as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![1,3,5,6], 5, 2)]
    #[case(vec![1,3,5,6], 2, 1)]
    #[case(vec![1,3,5,6], 7, 4)]
    fn search_insert(#[case] nums: Vec<i32>, #[case] target: i32, #[case] expected: i32) {
        assert_eq!(Solution::search_insert(nums, target), expected);
    }
}
