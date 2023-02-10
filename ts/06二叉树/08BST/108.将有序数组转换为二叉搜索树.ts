/**
 * /*
 *
 * [108] 将有序数组转换为二叉搜索树
 *
 * @format
 * @lc app=leetcode.cn id=108 lang=typescript
 */

// @lc code=start

class TreeNode {
  val: number
  left: TreeNode | null
  right: TreeNode | null
  constructor(val?: number, left?: TreeNode | null, right?: TreeNode | null) {
    this.val = val === undefined ? 0 : val
    this.left = left === undefined ? null : left
    this.right = right === undefined ? null : right
  }
}

function sortedArrayToBST(nums: number[]): TreeNode | null {
  function recur(nums: number[], start: number, end: number): TreeNode | null {
    if (start > end) return null
    let mid = Math.floor((start + end) / 2)
    const root = new TreeNode(nums[mid])
    root.left = recur(nums, start, mid - 1)
    root.right = recur(nums, mid + 1, end)
    return root
  }
  return recur(nums, 0, nums.length - 1)
}
// @lc code=end
export {}
