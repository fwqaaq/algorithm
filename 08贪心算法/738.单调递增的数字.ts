/**
 * /*
 *
 * [738] 单调递增的数字
 *
 * @format
 * @lc app=leetcode.cn id=738 lang=typescript
 */

// @lc code=start
function monotoneIncreasingDigits(n: number): number {
  let res: number[] = []
  let flag = 0
  while (n >= 1) {
    res.push(n % 10)
    n = Math.floor(n / 10)
  }
  for (let i = 1; i < res.length; i++) {
    if (res[i] > res[i - 1]) {
      // the position of the tag i,change every thing before i to 9
      flag = i
      res[i]--
    }
  }

  return res
    .map((item, index) => {
      if (index < flag) {
        return 9
      }
      return item
    })
    .reduce((pre, cur, index) => {
      return pre + cur * 10 ** index
    }, 0)
}
// @lc code=end

console.log(monotoneIncreasingDigits(132))

export {}
