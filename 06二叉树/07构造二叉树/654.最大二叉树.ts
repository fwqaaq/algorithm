/**
 * /*
 *
 * [654] 最大二叉树
 *
 * @format
 * @lc app=leetcode.cn id=654 lang=typescript
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

function constructMaximumBinaryTree(nums: number[]): TreeNode | null {
  if (nums.length === 0) return null
  let max = Math.max.apply(null, nums)
  let index = nums.indexOf(max)
  const node = new TreeNode(max)
  node.left = constructMaximumBinaryTree(nums.slice(0, index))
  node.right = constructMaximumBinaryTree(nums.slice(index + 1))
  return node
}
// @lc code=end

export {}
