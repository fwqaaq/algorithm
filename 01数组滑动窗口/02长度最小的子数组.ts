function minSubArrayLen(target: number, nums: number[]) {
  let left = 0,
    right = 0,
    sum = 0,
    min = Number.MAX_VALUE
  while (right < nums.length) {
    sum += nums[right++]
    while (sum >= target) {
      min = Math.min(min, right - left)
      sum -= nums[left++]
    }
  }
  return min === Number.MAX_VALUE ? 0 : min
}

console.log(minSubArrayLen(7, [2, 3, 1, 2, 4, 3]))
