/*
 * @lc app=leetcode.cn id=202 lang=typescript
 *
 * [202] 快乐数
 */

// @lc code=start
function isHappy(n: number): boolean {
  function update() {
    return String(n)
      .split("")
      .reduce((pre, num) => {
        return pre + Number(num) * Number(num)
      }, 0)
  }
  const set = new Set<number>()
  while (n !== 1 && !set.has(n)) {
    set.add(n)
    n = update()
  }
  return n === 1
}
// @lc code=end
console.log(isHappy(19))
