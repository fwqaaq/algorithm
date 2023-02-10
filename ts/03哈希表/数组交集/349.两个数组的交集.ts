/*
 * @lc app=leetcode.cn id=349 lang=typescript
 *
 * [349] 两个数组的交集
 */

// @lc code=start
function intersection(nums1: number[], nums2: number[]): number[] {
  return [...new Set(nums1)].filter((item) => nums2.includes(item))
}
// @lc code=end

console.log(intersection([1, 2, 2, 1], [2, 2]))
