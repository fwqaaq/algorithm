/**
 * /*
 *
 * [111] 二叉树的最小深度
 *
 * @format
 * @lc app=leetcode.cn id=111 lang=typescript
 */

// @lc code=start
/**
 * Definition for a binary tree node.
 * class TreeNode {
 *     val: number
 *     left: TreeNode | null
 *     right: TreeNode | null
 *     constructor(val?: number, left?: TreeNode | null, right?: TreeNode | null) {
 *         this.val = (val===undefined ? 0 : val)
 *         this.left = (left===undefined ? null : left)
 *         this.right = (right===undefined ? null : right)
 *     }
 * }
 */

function minDepth(root: TreeNode | null): number {
  // if (root === null) return 0
  // let left = minDepth(root.left) + 1
  // let right = minDepth(root.right) + 1
  // return left === 1 || right === 1
  //   ? left >= right
  //     ? left
  //     : right
  //   : Math.min(left, right)
  if (root === null) return 0
  let queue: TreeNode[] = [root]
  let count = 0
  while (queue.length !== 0) {
    let len = queue.length
    count++
    for (let i = 0; i < len; i++) {
      let curNode = queue.shift()
      if (curNode.left === null && curNode.right === null) return count
      if (curNode.left) queue.push(curNode.left)
      if (curNode.right) queue.push(curNode.right)
    }
  }
  return count
}
// @lc code=end
