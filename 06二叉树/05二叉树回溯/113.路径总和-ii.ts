/**
 * /*
 *
 * [113] 路径总和 II
 *
 * @format
 * @lc app=leetcode.cn id=113 lang=typescript
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

function pathSum(root: TreeNode | null, targetSum: number): number[][] {
  function recur(node: TreeNode, sum: number, path: number[]) {
    //* 深拷贝,因为回溯可能改变数组的内容
    if (!node.right && !node.left && sum === 0) return res.push([...path])
    if (node.left) {
      //* 递归
      sum -= node.left.val
      path.push(node.left.val)
      recur(node.left, sum, path)
      //* 不成功就回溯
      sum += node.left.val
      path.pop()
    }

    if (node.right)
      //* 隐藏的回溯,这里的数组使用深拷贝,不会影响到原数组
      recur(node.right, sum - node.right.val, [...path, node.right.val])
  }
  if (!root) return []
  let res: number[][] = []
  recur(root, targetSum - root.val, [root.val])
  return res
}
// @lc code=end

export {}
