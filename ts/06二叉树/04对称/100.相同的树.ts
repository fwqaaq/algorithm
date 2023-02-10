/**
 * /*
 *
 * [100] 相同的树
 *
 * @format
 * @lc app=leetcode.cn id=100 lang=typescript
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

function isSameTree(p: TreeNode | null, q: TreeNode | null): boolean {
  // 以下是递归的条件,只有所有条件符合才会进入递归
  // 即左右子树相等时
  //* 如果都是空,则返回true
  if (!q && !p) return true
  //* 如果一个是空,一个不是空,则返回false
  if (q === null || p === null) return false
  //* 判断两个节点是否相等
  if (q!.val !== p!.val) return false

  //* 递归的判断两棵树相同节点是否相等.
  let bool1 = isSameTree(q!.left, p!.left)
  let bool2 = isSameTree(q!.right, p!.right)

  return bool1 && bool2
}
// @lc code=end
export {}
