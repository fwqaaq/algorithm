/**
 * /*
 *
 * [701] 二叉搜索树中的插入操作
 *
 * @format
 * @lc app=leetcode.cn id=701 lang=typescript
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

function insertIntoBST(root: TreeNode | null, val: number): TreeNode | null {
  //* 递归的终止条件:找到null的节点,就返回
  if (!root) return new TreeNode(val)
  if (val < root.val) root.left = insertIntoBST(root.left, val)
  if (val > root.val) root.right = insertIntoBST(root.right, val)
  return root
}
// @lc code=end

export {}
