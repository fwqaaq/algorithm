function search(nums: number[], target: number): number {
  //return nums.indexOf(target) ?? -1

  let left = 0,
    right = nums.length - 1
  //左闭右闭[]
  while (left <= right) {
    let mid = Math.floor((left + right) / 2)
    //如果mid大于目标元素,那么左边的元素一定会有目标元素
    if (nums[mid] > target) {
      right = mid - 1 //因为是[],所以不用考虑越界
    } else if (nums[mid] < target) {
      left = mid + 1 //小于目标元素,那么右边的元素一定会有目标元素,并且不用考虑越界
    } else {
      return mid
    }
  }
  return -1
}
// @lc code=end
console.log(search([-1, 0, 3, 5, 9, 12], 9))
