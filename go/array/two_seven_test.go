package array

import "testing"

/*
 * @lc app=leetcode.cn id=27 lang=golang
 *
 * [27] 移除元素
 */

// @lc code=start
func removeElement(nums []int, val int) int {
	slow_index := 0
	for fast_index := 0; fast_index < len(nums); fast_index++ {
		if nums[fast_index] != val {
			nums[slow_index] = nums[fast_index]
			slow_index++
		}
	}
	return slow_index
}

// @lc code=end

func TestRemoveElement(t *testing.T) {
	nums := []int{3, 2, 2, 3}
	val := 3
	res := removeElement(nums, val)
	t.Log("I'm in test", res)
}
