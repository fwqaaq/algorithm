/**
 * /*
 *
 * [406] 根据身高重建队列
 *
 * @format
 * @lc app=leetcode.cn id=406 lang=typescript
 */

// @lc code=start
function reconstructQueue(people: number[][]): number[][] {
  const res: number[][] = []
  people
    .sort((a, b) => {
      //* 按照身高从大到小排序,相同则按照前面人数从少到多排序
      if (a[0] === b[0]) return a[1] - b[1]
      return b[0] - a[0]
    })
    .forEach((item) => {
      //* 根据前面的人数,插入到对应的位置
      res.splice(item[1], 0, item)
    })
  return res
}
// @lc code=end

console.log(
  reconstructQueue([
    [7, 0],
    [4, 4],
    [7, 1],
    [5, 0],
    [6, 1],
    [5, 2],
  ])
)

export {}
