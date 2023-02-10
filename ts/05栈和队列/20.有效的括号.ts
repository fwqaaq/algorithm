/*
 * @lc app=leetcode.cn id=20 lang=typescript
 *
 * [20] 有效的括号
 */

// @lc code=start
/**
 * Given a string, return true if the string is a valid set of parentheses
 * @param {string} s - string
 * @returns A boolean value.
 */
function isValid(s: string): boolean {
  let stack: string[] = []
  for (let i = 0; i < s.length; i++) {
    let char = s[i]
    switch (char) {
      case "(":
        stack.push(")")
        break
      case "{":
        stack.push("}")
        break
      case "[":
        stack.push("]")
        break
      default:
        if (stack.pop() !== char) {
          return false
        }
        break
    }
  }
  return stack.length === 0
}
// @lc code=end
