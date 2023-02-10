/**
 * /*
 *
 * [18] 四数之和
 *
 * @format
 * @lc app=leetcode.cn id=18 lang=typescript
 */

// @lc code=start
function fourSum(nums: number[], target: number): number[][] {
  let left: number, right: number
  let result: number[][] = []
  nums.sort((a, b) => a - b)
  for (let i = 0; i < nums.length - 3; i++) {
    if (i > 0 && nums[i] === nums[i - 1]) continue
    for (let j = i + 1; j < nums.length - 2; j++) {
      //这里的j范围一定是大于j+1
      if (j > i + 1 && nums[j] === nums[j - 1]) continue
      left = j + 1
      right = nums.length - 1
      //定义双指针边界,直到left ===right停止
      while (left < right) {
        let total = nums[i] + nums[j] + nums[left] + nums[right]
        let bool = total === target ? 0 : total > target ? -1 : 1
        switch (bool) {
          case 0:
            result.push([nums[i], nums[j], nums[left], nums[right]])
            left++
            right--
            while (nums[left] === nums[left - 1]) {
              left++
            }
            while (nums[right] === nums[right + 1]) {
              right--
            }
            break
          case 1:
            left++
            break
          case -1:
            right--
        }
      }
    }
  }
  return result
}
// @lc code=end
console.log(fourSum([2, 2, 2, 2, 2], 8))
