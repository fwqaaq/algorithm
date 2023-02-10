/**
 * /*
 *
 * [104] 二叉树的最大深度
 *
 * @format
 * @lc app=leetcode.cn id=104 lang=typescript
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

function maxDepth(root: TreeNode | null): number {
  if (root === null) return 0
  //* 左右子树遍历到最大深度才会停止,并且从下(从1开始)往上,每层+1
  //let leftDepth = maxDepth(root.left)+1
  //let rightDepth = maxDepth(root.right)+1
  //* 尾调用优化
  //return Math.max(leftDepth, rightDepth)
  let queue: TreeNode[] = [root]
  let count = 0
  while (queue.length !== 0) {
    let len = queue.length
    count++
    for (let i = 0; i < len; i++) {
      let curNode = queue.shift()
      if (curNode!.left) queue.push(curNode!.left)
      if (curNode!.right) queue.push(curNode!.right)
    }
  }
  return count
}
// @lc code=end
