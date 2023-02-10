/**
 * /*
 *
 * [110] 平衡二叉树
 *
 * @format
 * @lc app=leetcode.cn id=110 lang=typescript
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

function isBalanced(root: TreeNode | null): boolean {
  function recure(root: TreeNode | null): number {
    if (!root) return 0
    //* 返回左子节点的高度
    let left = recure(root.left)
    if (left === -1) return -1
    //* 返回右子节点的高度
    let right = recure(root.right)
    if (right === -1) return -1
    //* 比对当前递归左子节点的高度,如果大于1,那么返回-1,表示不是平衡二叉树
    if (Math.abs(left - right) > 1) return -1
    //* 只有当左右子节点高度不超过1,递归返回当前节点的高度
    return Math.max(left, right) + 1
  }
  return recure(root) !== -1
}
// @lc code=end

export {}
