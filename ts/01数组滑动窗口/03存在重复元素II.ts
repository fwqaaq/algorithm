function containsNearbyDuplicate(nums: number[], k: number): boolean {
  let right = 0
  let maps = new Map<number, number>()
  while (right < nums.length) {
    if (maps.has(nums[right]) && right - maps.get(nums[right]) <= k) return true
    maps.set(nums[right], right)
    right++
  }
  return false
}

console.log(containsNearbyDuplicate([1, 2, 3, 1, 2, 3], 2))
