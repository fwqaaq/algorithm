/*
 * @lc app=leetcode.cn id=383 lang=typescript
 *
 * [383] 赎金信
 */

// @lc code=start
function canConstruct(ransomNote: string, magazine: string): boolean {
  /* let hashArray = new Array(26).fill(0)
  let codeA = "a".charCodeAt(0)
  for (let i = 0; i < magazine.length; i++) {
    hashArray[magazine.charCodeAt(i) - codeA]--
    hashArray[ransomNote.charCodeAt(i) - codeA]++
  }
  return hashArray.every((item) => item <= 0) */
  let hashMap = new Map<string, number>()
  for (let i = 0; i < magazine.length; i++) {
    if (ransomNote[i] !== undefined) {
      hashMap.set(ransomNote[i], (hashMap.get(ransomNote[i]) || 0) + 1)
    }
    hashMap.set(magazine[i], (hashMap.get(magazine[i]) || 0) - 1)
  }
  return [...hashMap.values()].every((item) => item <= 0)
}
// @lc code=end
console.log(canConstruct("aa", "aab"))
