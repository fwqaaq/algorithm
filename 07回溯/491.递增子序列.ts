/**
 * /*
 *
 * [491] 递增子序列
 *
 * @format
 * @lc app=leetcode.cn id=491 lang=typescript
 */

// @lc code=start
function findSubsequences(nums: number[]): number[][] {
  const res: number[][] = []
  function trackback(startIndex: number, arr: number[]) {
    if (arr.length > 1) res.push([...arr])

    if (startIndex === nums.length) return
    for (let i = startIndex; i < nums.length; i++) {
      if (arr.length >= 1 && nums[i] < nums[i - 1]) continue
      arr.push(nums[i])
      trackback(i + 1, arr)
      arr.pop()
    }
  }
  trackback(0, [])
  return res
}
// @lc code=end
console.log(findSubsequences([4, 6, 7, 7]))
export {}
