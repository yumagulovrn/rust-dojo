/**
 * [0011] Container With Most Water
 *
 * You are given an integer array height of length n. There are n vertical lines drawn such that the two endpoints of the ith line are (i, 0) and (i, height[i]).
 * Find two lines that together with the x-axis form a container, such that the container contains the most water.
 * Return the maximum amount of water a container can store.
 * Notice that you may not slant the container.
 *
 * Example 1:
 * Input: height = [1,8,6,2,5,4,8,3,7]
 * Output: 49
 * Explanation: The above vertical lines are represented by array [1,8,6,2,5,4,8,3,7]. In this case, the max area of water (blue section) the container can contain is 49.
 *
 * Example 2:
 * Input: height = [1,1]
 * Output: 1
 *
 * Constraints:
 * * n == height.length
 * * 2 <= n <= 10^5
 * * 0 <= height[i] <= 10^4
 */
struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut max_area, mut l, mut r) = (0, 0, height.len() - 1);
        while l < r {
            max_area = max_area.max(height[l].min(height[r]) * (r - l) as i32);
            if height[l] < height[r] {
                l += 1;
            } else {
                r -= 1;
            }
        }
        max_area
    }

    pub fn max_area_iter(height: Vec<i32>) -> i32 {
        let mut max_area = 0;
        let mut i = height.iter().enumerate();
        let mut il = i.next();
        let mut ir = i.next_back();

        while let (Some((l, lh)), Some((r, rh))) = (il, ir) {
            max_area = max_area.max(lh.min(rh) * (r - l) as i32);
            if lh < rh {
                il = i.next();
            } else {
                ir = i.next_back();
            }
        }
        max_area
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![1, 8, 6, 2, 5, 4, 8, 3, 7], 49)]
    #[case(vec![1, 1], 1)]
    fn max_area(#[case] input: Vec<i32>, #[case] expected: i32) {
        assert_eq!(Solution::max_area(input), expected);
    }

    #[rstest]
    #[case(vec![1, 8, 6, 2, 5, 4, 8, 3, 7], 49)]
    #[case(vec![1, 1], 1)]
    fn max_area_iter(#[case] input: Vec<i32>, #[case] expected: i32) {
        assert_eq!(Solution::max_area_iter(input), expected);
    }
}
