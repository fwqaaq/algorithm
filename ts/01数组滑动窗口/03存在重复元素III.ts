function containsNearbyAlmostDuplicate(
  nums: number[],
  k: number,
  t: number
): boolean {
  for (let i = 0; i < nums.length; i++) {
    for (let j = 0; j < i; j++) {
      while (Math.abs(nums[j] - nums[i]) <= t && Math.abs(j - i) <= k) {
        return true
      }
    }
  }
  return false
}

console.log(containsNearbyAlmostDuplicate([1, 2, 3, 1], 3, 0))
