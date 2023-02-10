/*
 * @lc app=leetcode.cn id=347 lang=typescript
 *
 * [347] 前 K 个高频元素
 */

// @lc code=start
function topKFrequent(nums: number[], k: number): number[] {
  let confirmed: number[] = []
  const map: Map<number, number> = new Map()
  for (let i = 0; i < nums.length; i++) {
    //map.set(nums[i], map.get(nums[i]) ? map.get(nums[i])! + 1 : 1)
    map.set(nums[i], (map.get(nums[i]) || 0) + 1)
  }
  Array.from(map)
    .sort((a, b) => b[1] - a[1])
    .forEach(([key, value], index) => {
      if (index < k) {
        confirmed.push(key)
      }
    })
  return confirmed
}
// @lc code=end
console.log(topKFrequent([1, 3], 2))
