/**
 * /*
 *
 * [538] 把二叉搜索树转换为累加树
 *
 * @format
 * @lc app=leetcode.cn id=538 lang=typescript
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

function convertBST(root: TreeNode | null): TreeNode | null {
  if (!root) return null
  let sum = 0
  function recur(root: TreeNode) {
    if (!root) return null
    recur(root.right!)
    sum += root.val
    root.val = sum
    recur(root.left!)
  }
  recur(root)
  return root
}
// @lc code=end
export {}
