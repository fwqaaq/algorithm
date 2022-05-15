/**
 * /*
 *
 * [78] 子集
 *
 * @format
 * @lc app=leetcode.cn id=78 lang=typescript
 */

// @lc code=start
function subsets(nums: number[]): number[][] {
  const res: number[][] = []
  function trackback(startIndex: number, arr: number[]) {
    res.push([...arr])
    //剪枝优化,如果没有,for循环会return最后的函数
    if (startIndex >= nums.length) return

    for (let i = startIndex; i < nums.length; i++) {
      arr.push(nums[i])
      trackback(i + 1, arr)
      arr.pop()
    }
  }
  trackback(0, [])
  return res
}
// @lc code=end

console.log(subsets([1, 2, 3]))
export {}
