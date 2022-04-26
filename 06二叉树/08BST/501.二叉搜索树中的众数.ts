/**
 * /*
 *
 * [501] 二叉搜索树中的众数
 *
 * @format
 * @lc app=leetcode.cn id=501 lang=typescript
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

function findMode(root: TreeNode | null): number[] {
  let res: number[] = []
  let maxCount = 1
  let count = 0
  let pre: TreeNode = null
  function inOrder(root: TreeNode | null) {
    if (!root) return
    inOrder(root.left)
    //* 只有第一次回溯之后才会记录
    if (pre === null || pre.val !== root.val) count = 1
    //* 如果当前节点和前一个节点相同,则count++
    if (root.val === pre.val) count++
    pre = root
    if (count === maxCount) res.push(root.val)
    //* 只要当前节点的count大于maxCount,则更新maxCount
    if (count > maxCount) {
      maxCount = count
      //* 清空数组
      res.length = 0
      res.push(root.val)
    }
    inOrder(root.right)
  }
  inOrder(root)
  return res
}
// @lc code=end
const root = new TreeNode(1, null, new TreeNode(2, new TreeNode(2)))
console.log(findMode(root))
export {}
