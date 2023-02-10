/**
 * /*
 *
 * [90] 子集 II
 *
 * @format
 * @lc app=leetcode.cn id=90 lang=typescript
 */

// @lc code=start
function subsetsWithDup(nums: number[]): number[][] {
  nums.sort((a, b) => a - b)
  const res: number[][] = []
  function trackback(startIndex: number, arr: number[]) {
    res.push([...arr])
    //* 当开头的索引到数组的末尾时,该集合就已经结束
    if (startIndex === nums.length) return
    for (let i = startIndex; i < nums.length; i++) {
      //* 只有广度遍历才会出现重复的情况,i在本次回溯变化才能出现广度遍历
      //并且此时的nums[i]不能与之前的i重复nums[i - 1]
      if (i > startIndex && nums[i] === nums[i - 1]) continue
      arr.push(nums[i])
      trackback(i + 1, arr)
      arr.pop()
    }
  }
  trackback(0, [])
  return res
}
// @lc code=end
export {}
