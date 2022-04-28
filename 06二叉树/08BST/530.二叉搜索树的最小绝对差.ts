/**
 * /*
 *
 * [530] 二叉搜索树的最小绝对差
 *
 * @format
 * @lc app=leetcode.cn id=530 lang=typescript
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

function getMinimumDifference(root: TreeNode | null): number {
  function inOrder(root: TreeNode | null) {
    if (!root) return
    inOrder(root.left)
    res.push(root.val)
    inOrder(root.right)
    // return root ? [...inOrder(root.left), root.val, ...inOrder(root.right)] : []
  }
  const res: number[] = []
  inOrder(root)
  let min = Infinity
  for (let i = 0; i < res.length - 1; i++) {
    min = Math.min(min, Math.abs(res[i + 1] - res[i]))
  }
  return min
}
// @lc code=end

export {}
