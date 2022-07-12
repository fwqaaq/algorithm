/**
 * /*
 *
 * [763] 划分字母区间
 *
 * @format
 * @lc app=leetcode.cn id=763 lang=typescript
 */

// @lc code=start
function partitionLabels(s: string): number[] {
  const alphabet = new Array(26).fill(0)
  const res: number[] = []
  let right = 0
  let left = -1
  s.split('')
    .map((item, index) => {
      // 更新每个元素的最远的右边界
      alphabet[item.charCodeAt(0) - 97] = index
      return item
    })
    .forEach((item, index) => {
      // 最远右边界
      right = Math.max(right, alphabet[item.charCodeAt(0) - 97])
      // 如果已经到达最远右边界，则说明该字母已经被分割
      if (index === right) {
        res.push(right - left)
        left = right
      }
    })
  return res
}
// @lc code=end

console.log(partitionLabels('ababcbacadefegdehijhklij'))

export {}
