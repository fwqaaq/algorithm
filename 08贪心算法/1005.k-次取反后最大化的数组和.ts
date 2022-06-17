/**
 * /*
 *
 * [1005] K 次取反后最大化的数组和
 *
 * @format
 * @lc app=leetcode.cn id=1005 lang=typescript
 */

// @lc code=start
function largestSumAfterKNegations(nums: number[], k: number) {
  //按绝对值的大小从大到小排序
  nums.sort((a, b) => Math.abs(b) - Math.abs(a))
  let cur = 0
  while (k > 0 && cur < nums.length) {
    if (nums[cur] < 0) {
      //从前面取反,只有最大的负数取反
      nums[cur] = -nums[cur]
      k--
    }
    cur++
  }
  if (k % 2 === 1) {
    //如果k为奇数,取反最小值
    nums[nums.length - 1] *= -1
  }
  return nums.reduce((a, b) => a + b, 0)
}
// @lc code=end
console.log(largestSumAfterKNegations([-2, 5, 0, 2, -2], 3))
export {}
