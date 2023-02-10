/**
 * /*
 *
 * [235] 二叉搜索树的最近公共祖先
 *
 * @format
 * @lc app=leetcode.cn id=235 lang=typescript
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

function lowestCommonAncestor(
  root: TreeNode | null,
  p: TreeNode | null,
  q: TreeNode | null
): TreeNode | null {
  if (root === null) return root
  if (root.val > p!.val && root.val > q!.val)
    return lowestCommonAncestor(root.left, p, q)
  if (root.val < p!.val && root.val < q!.val)
    return lowestCommonAncestor(root.right, p, q)
  return root
}
// @lc code=end

export {}
