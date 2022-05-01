/**
 * /*
 *
 * [222] 完全二叉树的节点个数
 *
 * @format
 * @lc app=leetcode.cn id=222 lang=typescript
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

function countNodes(root: TreeNode | null): number {
  if (root === null) return 0
  // return countNodes(root.left) + countNodes(root.right) + 1
  let stack: TreeNode[] = [root]
  let count = 1
  while (stack.length) {
    const node = stack.pop()
    if (node!.right) {
      count++
      stack.push(node!.right)
    }
    if (node!.left) {
      count++
      stack.push(node!.left)
    }
  }
  return count
}
// @lc code=end
export{}