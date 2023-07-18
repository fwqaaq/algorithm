/*

* @lc app=leetcode.cn id=34 lang=rust
*
* [34] 在排序数组中查找元素的第一个和最后一个位置
*/

use std::cmp::Ordering;

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // let mut left = 0;
        // let mut right = nums.len();
        // while left < right {
        //     let mid = (left + right) / 2;
        //     match nums[mid].cmp(&target) {
        //         Ordering::Less => left = mid + 1,
        //         Ordering::Greater => right = mid,
        //         Ordering::Equal => {
        //             let mut l = mid;
        //             let mut r = mid;
        //             while l > 0 && nums[l - 1] == target {
        //                 l -= 1;
        //             }
        //             while r < nums.len() - 1 && nums[r + 1] == target {
        //                 r += 1;
        //             }
        //             return vec![l as i32, r as i32];
        //         }
        //     }
        // }
        let right_border = Solution::get_right_border(&nums, target);
        let left_border = Solution::get_left_border(&nums, target);
        if right_border == -2 || left_border == -2 {
            return vec![-1, -1];
        }
        if right_border - left_border > 0 {
            return vec![left_border, right_border - 1];
        }
        vec![-1, -1]
    }

    pub fn get_right_border(nums: &Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right: usize = nums.len();
        let mut right_border: i32 = -2;
        while left < right {
            let mid = (left + right) / 2;
            if nums[mid] > target {
                right = mid;
            } else {
                left = mid + 1;
                println!("left{},right{}", left, right);
                right_border = left as i32;
            }
        }
        right_border
    }

    pub fn get_left_border(nums: &Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len();
        let mut left_border: i32 = -2;
        while left < right {
            let mid = (left + right) / 2;
            if nums[mid] >= target {
                right = mid;
                left_border = right as i32;
            } else {
                left = mid + 1;
            }
        }
        left_border
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_range_test() {
        let nums = vec![5, 7, 7, 8, 8, 10];
        let target = 6;
        let result = Solution::search_range(nums, target);
        print!("{:?}", result);
        // assert_eq!(result, vec![2, 3]);
    }
}
