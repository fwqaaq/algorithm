/**
 * /*
 *
 * [700] 二叉搜索树中的搜索
 *
 * @format
 * @lc app=leetcode.cn id=700 lang=typescript
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

function searchBST(root: TreeNode | null, val: number): TreeNode | null {
  if (!root) return null
  if (val > root.val) return searchBST(root.right, val)
  if (val < root.val) return searchBST(root.left, val)
  return root
}
// @lc code=end
export {}
