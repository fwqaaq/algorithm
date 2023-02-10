/**
 * /*
 *
 * [106] 从中序与后序遍历序列构造二叉树
 *
 * @format
 * @lc app=leetcode.cn id=106 lang=typescript
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

function buildTree(inorder: number[], postorder: number[]): TreeNode | null {
  if (inorder.length === 0) return null
  const node = new TreeNode(postorder.pop())
  const index = inorder.indexOf(node.val)
  //* 切割数组.
  //* 后序遍历的最后一个元素是子树的根节点
  //* 中序遍历的根节点分开的是左右子树
  node.left = buildTree(inorder.slice(0, index), postorder.slice(0, index))
  node.right = buildTree(inorder.slice(index + 1), postorder.slice(index))
  return node
}
// @lc code=end

export {}
