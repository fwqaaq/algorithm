/**
 * /*
 *
 * [669] 修剪二叉搜索树
 *
 * @format
 * @lc app=leetcode.cn id=669 lang=typescript
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

function trimBST(
  root: TreeNode | null,
  low: number,
  high: number
): TreeNode | null {
  //* 递归的终止条件
  if (!root) return null
  //* 如果当前节点小于low,那么就返回右子树
  if (root.val < low) return trimBST(root.right, low, high)
  //* 如果当前节点大于high,那么就返回左子树
  if (root.val > high) return trimBST(root.left, low, high)
  root.left = trimBST(root.left, low, high)
  root.right = trimBST(root.right, low, high)
  return root
}
// @lc code=end
export {}
