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
    const set = new Set<number>()
    for (let i = startIndex; i < nums.length; i++) {
      //每次回溯的set都是新的,为了防止广度遍历搜索的重复,需要去使用set去重
      //将当前的数字加入set中,回溯的时候如果有数字则跳过本次
      if (set.has(nums[i]) || nums[i] < arr[arr.length - 1]) continue
      set.add(nums[i])
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
