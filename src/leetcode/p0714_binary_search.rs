/**
 * [0714] Binary Search
 *
 * Given an array of integers nums which is sorted in ascending order, and an integer target, write a function to search target in nums. If target exists, then return its index. Otherwise, return -1.
 *
 * You must write an algorithm with O(log n) runtime complexity.
 *
 * Example 1:
 * Input: nums = [-1,0,3,5,9,12], target = 9
 * Output: 4
 * Explanation: 9 exists in nums and its index is 4
 *
 * Example 2:
 * Input: nums = [-1,0,3,5,9,12], target = 2
 * Output: -1
 * Explanation: 2 does not exist in nums so return -1
 *
 * Constraints:
 * * 1 <= nums.length <= 104
 * * -104 < nums[i], target < 104
 * * All the integers in nums are unique.
 * * nums is sorted in ascending order.
 */
use std::cmp::Ordering;
struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut low, mut high) = (0, nums.len());
        while low < high {
            let mid = low + (high - low) / 2;
            match nums[mid].cmp(&target) {
                Ordering::Equal => return mid as i32,
                Ordering::Less => low = mid + 1,
                Ordering::Greater => high = mid,
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![-1, 0, 3, 5, 9, 12], 9, 4)]
    #[case(vec![-1, 0, 3, 5, 9, 12], 2, -1)]
    fn search(#[case] nums: Vec<i32>, #[case] target: i32, #[case] expected: i32) {
        assert_eq!(Solution::search(nums, target), expected);
    }
}
