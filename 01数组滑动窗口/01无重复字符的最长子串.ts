/** @format */

function lengthOfLongestSubstring(s: string) {
  let l = 0
  let max = 0
  let map = new Map<string, number>()
  if (s.length === 0) return 0
  for (let i = 0; i < s.length; i++) {
    if (map.has(s.charAt(i))) {
      l = Math.max(l, map.get(s.charAt(i))! + 1)
    }
    map.set(s.charAt(i), i)
    max = Math.max(max, i - l + 1)
  }
  return max
}

console.log(lengthOfLongestSubstring("abcabcbb"))
