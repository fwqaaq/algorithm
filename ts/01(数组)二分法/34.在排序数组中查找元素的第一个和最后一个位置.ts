/*
 * @lc app=leetcode.cn id=34 lang=typescript
 *
 * [34] 在排序数组中查找元素的第一个和最后一个位置
 */

// @lc code=start
function searchRange(nums: number[], target: number) {
  let index = nums.indexOf(target)
  if (index === -1) {
    return [-1, -1]
  }
  let left = index
  while (nums[left] === target) {
    left++
  }
  return [index, left - 1]
}
// @lc code=end

console.log(searchRange([5, 7, 7, 8, 8, 8, 8, 8, 8, 8, 8, 10], 8))
