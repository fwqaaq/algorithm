/*
 * @lc app=leetcode.cn id=151 lang=typescript
 *
 * [151] 颠倒字符串中的单词
 */

// @lc code=start
/**
 * Given a string, reverse the order of the words in the string
 * @param {string} s - string
 * @returns The reversed string.
 */
function reverseWords(s: string) {
  //return s.trim().replace(/\s+/g, " ").split(" ").reverse().join(" ")
  let left = 0,
    right = 0,
    str = ""
  let arr: string[] = []
  for (let i = 0; i < s.length; i++) {
    if (s[i] === " ") {
      continue
    }
    //todo:只要不是空格,left指针指向当前的字符
    left = i
    while (s[i] !== " " && i < s.length) {
      i++
      //todo:如果s[i]是空格,right指针指向当前的字符
      right = i
    }
    arr.push(s.substring(left, right))
  }
  for (let i = arr.length - 1; i >= 0; i--) {
    if (i !== 0) arr[i] += " "
    str += arr[i]
  }
  return str
}
// @lc code=end
console.log(reverseWords("  the sky is   blue  "))
