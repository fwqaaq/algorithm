/*
 * @lc app=leetcode.cn id=242 lang=typescript
 *
 * [242] 有效的字母异位词
 */

// @lc code=start
function isAnagram(s: string, t: string) {
  if (s.length !== t.length) return false
  let hashArray: number[] = new Array(26).fill(0)
  let indexCode: number = "a".charCodeAt(0)
  for (let i = 0; i < s.length; i++) {
    hashArray[s.charCodeAt(i) - indexCode]++
    hashArray[t.charCodeAt(i) - indexCode]--
  }
  //如果每一项都是0,说明字母是异位的
  return hashArray.every((item) => item === 0)
}
isAnagram("anag", "nagaram")
// @lc code=end
