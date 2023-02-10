/**
 * /*
 *
 * [968] 监控二叉树
 *
 * @format
 * @lc app=leetcode.cn id=968 lang=typescript
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

function minCameraCover(root: TreeNode | null): number {
  /** 0-无覆盖， 1-有摄像头， 2-有覆盖 */
  type status = 0 | 1 | 2
  let res = 0
  if (recur(root) === 0) res++
  function recur(root: TreeNode | null): status {
    if (!root) return 2
    const left = recur(root.left),
      right = recur(root.right)
    let resStatus: status = 0
    if (left === 0 || right === 0) {
      res++
      resStatus = 1
    } else if (left === 1 || right === 1) {
      resStatus = 2
    } else {
      resStatus = 0
    }
    return resStatus
  }
  return res
}
// @lc code=end

export {}
