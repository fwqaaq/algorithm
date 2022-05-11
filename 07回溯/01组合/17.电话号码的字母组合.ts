/**
 * /*
 *
 * [17] 电话号码的字母组合
 *
 * @format
 * @lc app=leetcode.cn id=17 lang=typescript
 */

// @lc code=start
function letterCombinations(digits: string): string[] {
  const dictonary = [
    "",
    "",
    "abc",
    "def",
    "ghi",
    "jkl",
    "mno",
    "pqrs",
    "tuv",
    "wxyz",
  ]
  const result: string[] = []
  function backtrack(current: string, next: number, str: string) {
    if (digits.length === 0) return []
    //* 终止条件,达到规定的映射长度后,return
    if (next === digits.length) {
      result.push(str)
      return
    }

    //* 每次递归时,取出当前的字符串
    const temp = dictonary[Number(digits[next])]
    for (let i = 0; i < temp.length; i++) {
      str += temp[i]
      backtrack(current, next + 1, str)
      //* 回溯,将当前成功的字符删除,执行下一次递归
      str = str.slice(0, str.length - 1)
    }
  }
  backtrack(digits, 0, "")

  return result
}
// @lc code=end

letterCombinations("23")
export {}
