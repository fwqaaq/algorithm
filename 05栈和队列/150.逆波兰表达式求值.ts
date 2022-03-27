/*
 * @lc app=leetcode.cn id=150 lang=typescript
 *
 * [150] 逆波兰表达式求值
 */

// @lc code=start
/**
 * Given a string array of tokens, return the result of evaluating the tokens as a RPN expression
 * @param {string[]} tokens - an array of strings
 * @returns The result of the expression.
 */
function evalRPN(tokens: string[]): number {
  let stack: string[] = []
  const map: Map<string, (a: number, b: number) => number> = new Map()
  map.set("+", (a, b) => a + b)
  map.set("-", (a, b) => a - b)
  map.set("*", (a, b) => a * b)
  map.set("/", (a, b) => parseInt(`${a / b}`))
  for (let i = 0; i < tokens.length; i++) {
    if (map.has(tokens[i])) {
      const b = Number(stack.pop())
      const a = Number(stack.pop())
      stack.push(map.get(tokens[i])!(a, b).toString())
    } else {
      stack.push(tokens[i])
    }
  }
  return Number(stack.pop())
}
// @lc code=end
console.log(
  evalRPN(["10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"])
)
