/**
 * /*
 *
 * [105] 从前序与中序遍历序列构造二叉树
 *
 * @format
 * @lc app=leetcode.cn id=105 lang=typescript
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

function buildTree(preorder: number[], inorder: number[]): TreeNode | null {
  if (inorder.length === 0) return null
  const node = new TreeNode(preorder[0])
  const index = inorder.indexOf(node.val)
  node.left = buildTree(preorder.slice(1, index + 1), inorder.slice(0, index))
  node.right = buildTree(preorder.slice(index + 1), inorder.slice(index + 1))
  return node
}
// @lc code=end

export {}
