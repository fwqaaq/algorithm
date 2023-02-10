/**
 * /*
 *
 * [56] 合并区间
 *
 * @format
 * @lc app=leetcode.cn id=56 lang=typescript
 */

// @lc code=start
function merge(intervals: number[][]): number[][] {
  const res: number[][] = []

  let end: number
  intervals
    .sort((a, b) => a[0] - b[0])
    .forEach((item, index) => {
      if (index === 0) {
        res[0] = [...intervals[0]]
        return (end = item[1])
      }
      if (item[0] <= end) {
        // find the maximum end position
        end = Math.max(end, item[1])
      } else {
        res[res.length - 1][1] = end
        res.push([...item])
        // Updates if the current start position is greater than the previoes end position
        end = item[1]
      }
    })
  return res
}
// @lc code=end
console.log(
  merge([
    [1, 4],
    [0, 4],
  ])
)
export {}
