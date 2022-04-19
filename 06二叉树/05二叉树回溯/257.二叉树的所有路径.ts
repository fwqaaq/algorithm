/**
 * /*
 *
 * [257] 二叉树的所有路径
 *
 * @format
 * @lc app=leetcode.cn id=257 lang=typescript
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

function binaryTreePaths(root: TreeNode | null): string[] {
  function recure(root: TreeNode | null, route: string, resArr: string[]) {
    if (!root) return
    route += root.val
    //* 如果是叶子节点,表示当前路径已经结束,添加到结果数组中
    if (!root.left && !root.right) {
      resArr.push(route)
      return
    }
    //* 回溯,先遍历左子树,再回溯右子树(前序遍历)
    if (root.left) recure(root.left, "->", resArr)
    if (root.right) recure(root.right, "->", resArr)
  }
  let resArr: string[] = []
  recure(root, "", resArr)
  return resArr
}
// @lc code=end

export {}
