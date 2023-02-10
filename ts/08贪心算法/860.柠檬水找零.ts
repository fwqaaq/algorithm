/**
 * /*
 *
 * [860] 柠檬水找零
 *
 * @format
 * @lc app=leetcode.cn id=860 lang=typescript
 */

// @lc code=start
function lemonadeChange(bills: number[]): boolean {
  let five = 0
  let ten = 0
  for (let i = 0; i < bills.length; i++) {
    switch (bills[i]) {
      case 5:
        five++
        break
      case 10:
        if (five <= 0) return false
        five--
        ten++
      case 20:
        if (bills[i] === 20) {
          if (ten > 0 && five > 0) {
            ten--
            five--
          } else if (five >= 3) {
            five -= 3
          } else return false
        }
    }
  }
  return true
}
// @lc code=end

export {}
