/**
 * /*
 *
 * [404] 左叶子之和
 *
 * @format
 * @lc app=leetcode.cn id=404 lang=typescript
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

function sumOfLeftLeaves(root: TreeNode | null): number {
  if (!root) return 0
  let sum = 0
  //* 只有是叶子节点(左右子树都为空)才算左叶子
  if (root.left !== null && root.left.left === null && root.left.right === null)
    sum += root.left.val
  return sum + sumOfLeftLeaves(root.left) + sumOfLeftLeaves(root.right)
}
// @lc code=end
export {}
