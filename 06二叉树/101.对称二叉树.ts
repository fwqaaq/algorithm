/**
 * /*
 *
 * [101] 对称二叉树
 *
 * @format
 * @lc app=leetcode.cn id=101 lang=typescript
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

function isSymmetric(root: TreeNode | null): boolean {
  // if (!root) return true
  // function recure(left: TreeNode | null, right: TreeNode | null): boolean {
  //   //* 如果两个数同时为空,则返回true
  //   if (!left && !right) return true
  //   //* 如果两个数不为空,则返回false
  //   if (!left || !right) return false
  //   if (left.val !== right.val) return false
  //   return recure(left.left, right.right) && recure(left.right, right.left)
  // }
  // return recure(root.left, root.right)

  if (!root) return true
  let stack: TreeNode[] = [root.left!, root.right!]
  while (stack.length) {
    let right = stack.pop()!
    let left = stack.pop()!
    if (!left && !right) continue
    if (!left || !right) return false
    if (left.val !== right.val) return false
    stack.push(left.left!)
    stack.push(right.right!)
    stack.push(left.right!)
    stack.push(right.left!)
  }
  return true
}
// @lc code=end

const root: TreeNode = new TreeNode(1)
root.left = new TreeNode(2)
root.right = new TreeNode(2)
root.left.left = new TreeNode(3)
root.left.right = new TreeNode(4)
root.right.left = new TreeNode(4)
root.right.right = new TreeNode()

isSymmetric(root)

export {}
