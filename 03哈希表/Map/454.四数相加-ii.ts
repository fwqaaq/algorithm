/*
 * @lc app=leetcode.cn id=454 lang=typescript
 *
 * [454] 四数相加 II
 */

// @lc code=start
function fourSumCount(
  nums1: number[],
  nums2: number[],
  nums3: number[],
  nums4: number[]
): number {
  const map = new Map<number, number>()
  let tempValue: number | undefined
  let total = 0
  for (let num1 of nums1) {
    for (let num2 of nums2) {
      map.set(num1 + num2, map.get(num1 + num2)! + 1 || 1)
    }
  }
  for (let num3 of nums3) {
    for (let num4 of nums4) {
      tempValue = map.get(0 - num3 - num4)
      if (tempValue) {
        total += tempValue
      }
    }
  }
  return total
}
// @lc code=end
