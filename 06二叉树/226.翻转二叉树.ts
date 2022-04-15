/**
 * /*
 *
 * [226] 翻转二叉树
 *
 * @format
 * @lc app=leetcode.cn id=226 lang=typescript
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

function invertTree(root: TreeNode | null): TreeNode | null {
  // if (!root) return null
  // let temp = root.left
  // root.left = root.right
  // root.right = temp
  // invertTree(root.left)
  // invertTree(root.right)
  // return root
  if (!root) return null
  let stack: TreeNode[] = [root]
  //* 对root的引用
  let curNode: TreeNode | null
  while (stack.length) {
    curNode = stack.pop()!
    if (curNode.right) stack.push(curNode.right)
    if (curNode.left) stack.push(curNode.left)
    let temp = curNode.left
    curNode.left = curNode.right
    curNode.right = temp
  }
  return root
}
// @lc code=end

export {}
