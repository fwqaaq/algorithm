/*
 * @lc app=leetcode.cn id=239 lang=typescript
 *
 * [239] 滑动窗口最大值
 */

// @lc code=start
/**
 * Find the maximum value in a sliding window of length k
 * @param {number[]} nums - an array of integers
 * @param {number} k - The size of the sliding window.
 * @returns The maximum value of each subarray of length k.
 */
function maxSlidingWindow(nums: number[], k: number): number[] {
  let arr: number[] = []
  //* 因为i是从滑动窗口的长度的索引位置开始,所以i必须直到等于nums.length+1才能结束
  for (let i = k; i <= nums.length; i++) {
    let left = i - k
    let max = -Number.MAX_VALUE
    while (left < i) {
      max = Math.max(max, nums[left])
      left++
    }
    arr.push(max)
  }
  return arr
}
// @lc code=end
console.log(maxSlidingWindow([1, 3, -1, -3, 5, 3, 6, 7], 3))
