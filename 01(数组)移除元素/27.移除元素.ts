/*
 * @lc app=leetcode.cn id=27 lang=typescript
 *
 * [27] 移除元素
 */

// @lc code=start
function removeElement(nums: number[], val: number): number {
  let low = 0
  let high = 0
  while (high < nums.length) {
    //如果high指针指向的值符合target,将最前面的值依次变为target
    if (nums[high] !== val) nums[low++] = nums[high]
    high++
  }
  nums[low] = val
  console.log(nums)
  return low
}
// @lc code=end

console.log(removeElement([1, 2, 2, 3, 4, 3], 3))
