/**
 * /*
 *
 * [45] 跳跃游戏 II
 *
 * @format
 * @lc app=leetcode.cn id=45 lang=typescript
 */

// @lc code=start
function jump(nums: number[]): number {
  if (nums.length === 1) return 0
  let cur = 0
  let next = 0
  let step = 0
  //* i的移动位置最远只能在nums.length - 2的位置
  //* 因为只需要step+1就能到最后一个位置了
  for (let i = 0; i < nums.length - 1; i++) {
    next = Math.max(next, i + nums[i])
    //* 每次只要走到当前范围内的最大值,步数++
    if (i === cur) {
      //遇到当前覆盖的最远距离下标
      cur = next //更新当前覆盖的最远距离下标
      step++
    }
  }
  return step
}
// @lc code=end
console.log(jump([2, 2, 0, 1, 4]))
export {}
