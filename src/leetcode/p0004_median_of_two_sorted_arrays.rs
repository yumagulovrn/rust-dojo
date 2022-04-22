/**
 * [0004] Median of Two Sorted Arrays
 *
 * Given two sorted arrays nums1 and nums2 of size m and n respectively, return the median of the two sorted arrays.
 * The overall run time complexity should be O(log (m+n)).
 *
 * Example 1:
 * Input: nums1 = [1,3], nums2 = [2]
 * Output: 2.00000
 * Explanation: merged array = [1,2,3] and median is 2.
 *
 * Example 2:
 * Input: nums1 = [1,2], nums2 = [3,4]
 * Output: 2.50000
 * Explanation: merged array = [1,2,3,4] and median is (2 + 3) / 2 = 2.5.
 *
 * Constraints:
 * * nums1.length == m
 * * nums2.length == n
 * * 0 <= m <= 1000
 * * 0 <= n <= 1000
 * * 1 <= m + n <= 2000
 * * -10^6 <= nums1[i], nums2[i] <= 10^6
 */
struct Solution;

struct SortedVectors {
    vec1: Vec<i32>,
    vec2: Vec<i32>,
}

impl SortedVectors {
    fn len(&self) -> usize {
        self.vec1.len() + self.vec2.len()
    }
}

impl Iterator for SortedVectors {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        match (self.vec1.last(), self.vec2.last()) {
            (Some(n1), Some(n2)) if n1 >= n2 => self.vec1.pop(),
            (Some(_), Some(_)) => self.vec2.pop(),
            (Some(_), _) => self.vec1.pop(),
            (_, Some(_)) => self.vec2.pop(),
            _ => None,
        }
    }
}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut vecs = SortedVectors {
            vec1: nums1,
            vec2: nums2,
        };
        let length = vecs.len();
        let middle = length / 2;

        if length % 2 == 0 {
            let mut halves = vecs.skip(middle - 1);
            (halves.next().unwrap() as f64 + halves.next().unwrap() as f64) / 2.0
        } else {
            vecs.nth(middle).unwrap() as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![1, 3], vec![2], 2.0)]
    #[case(vec![1, 2], vec![3, 4], 2.5)]
    fn find_median_sorted_arrays(
        #[case] nums1: Vec<i32>,
        #[case] nums2: Vec<i32>,
        #[case] expected: f64,
    ) {
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), expected);
    }
}
