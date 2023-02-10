/*
 * @lc app=leetcode.cn id=15 lang=typescript
 *
 * [15] 三数之和
 */

// @lc code=start
function threeSum(nums: number[]): number[][] {
  let numArr: number[][] = []
  //初始化左右两个指针
  let left: number, right: number
  //排序
  nums.sort((a, b) => a - b)
  for (let i = 0; i < nums.length; i++) {
    if (i > 0 && nums[i] === nums[i - 1]) {
      continue
    }
    //初始化左右指针
    left = i + 1
    right = nums.length - 1
    //定义边界做右指针边界
    while (left < right) {
      let total = nums[i] + nums[right] + nums[left]
      let bool = total > 0 ? 1 : total < 0 ? -1 : 0
      switch (bool) {
        case 0:
          numArr.push([nums[i], nums[left], nums[right]])
          left++
          right--
          //只要左右指针的值相同，就跳过
          while (nums[left] === nums[left - 1]) {
            left++
          }
          while (nums[right] === nums[right + 1]) {
            right--
          }
          break
        case -1:
          left++
          break
        case 1:
          right--
          break
      }
    }
  }
  return numArr
}

// @lc code=end
console.log(threeSum([-1, 0, 1, 2, -1, -4]))
