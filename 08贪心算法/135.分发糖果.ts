/**
 * /*
 *
 * [135] 分发糖果
 *
 * @format
 * @lc app=leetcode.cn id=135 lang=typescript
 */

// @lc code=start
function candy(ratings: number[]): number {
  const res: number[] = []
  res[0] = 1

  // 从左向右遍历,确定右边的糖果>左边低分的糖果
  //* walk from left to right to make sure the candy on the right > low score candy on the left
  for (let i = 1; i < ratings.length; i++) {
    if (ratings[i] > ratings[i - 1]) {
      res[i] = res[i - 1] + 1
    } else {
      res[i] = 1
    }
  }

  //* walk from right to left to make sure the candy on the left > low sorce candy on the right
  for (let i = res.length - 2; i >= 0; i--) {
    if (ratings[i] > ratings[i + 1]) res[i] = Math.max(res[i], res[i + 1] + 1)
  }

  return res.reduce((a, b) => a + b, 0)
}
// @lc code=end
