/**
 * /*
 *
 * [435] 无重叠区间
 *
 * @format
 * @lc app=leetcode.cn id=435 lang=typescript
 */

// @lc code=start 求最多的非重叠区间
function eraseOverlapIntervals(intervals: number[][]): number {
  // 至少有一个区间不重叠
  let intervalCount = 1
  // 不重叠区间分割点
  let end: number
  intervals
    .sort((a, b) => {
      if (a[1] === b[1]) return a[0] - b[0]
      return a[1] - b[1]
    })
    .forEach((item, index) => {
      if (index === 0) end = item[1]
      //更新:只要当前的开头时间大于上一个分割点,则增加 1
      if (item[0] >= end) {
        intervalCount++
        //更新区间分割点
        end = item[1]
      }
    })
  return intervals.length - intervalCount
}
// @lc code=end

console.log(
  eraseOverlapIntervals([
    [1, 100],
    [11, 22],
    [1, 11],
    [2, 12],
  ])
)

export {}
