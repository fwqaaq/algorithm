function maxSlidingWindow(nums: number[], k: number): number[] {
  let complyNums: number[] = []

  for (let i = k; i <= nums.length; i++) {
    //每次右指针移动窗口时,都初始化为最小值
    let max: number = -Number.MAX_VALUE
    //移动左指针(左右指针应保持滑动窗口的大小为k)
    let left = i - k
    while (left < i) {
      //每次比较取最大值
      max = Math.max(max, nums[left])
      left++
    }
    complyNums.push(max)
  }
  return complyNums
}

console.log(maxSlidingWindow([1, 3, -1, -3, 5, 3, 6, 7], 3))
export {}
