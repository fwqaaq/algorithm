/*
 * @lc app=leetcode.cn id=49 lang=typescript
 *
 * [49] 字母异位词分组
 */

// @lc code=start
function groupAnagrams(strs: string[]): string[][] {
  if (strs.length === 0) return [[""]]
  let array: string[] = []
  let s = [...strs]
  for (let i = 0; i < strs.length; i++) {
    strs[i] = strs[i].split("").sort().join("")
    array.push(strs[i])
  }
  const set = new Set(array)
  const result: string[][] = []
  for (let v of set.keys()) {
    const temp: string[] = []
    for (let i = 0; i < array.length; i++) {
      if (array[i] === v) {
        temp.push(s[i])
      }
    }
    result.push(temp)
  }
  return result
}
// @lc code=end
groupAnagrams(["eat", "tea", "tan", "ate", "nat", "bat"])
