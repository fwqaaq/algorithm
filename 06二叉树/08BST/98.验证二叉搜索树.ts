/**
 * /*
 *
 * [98] 验证二叉搜索树
 *
 * @format
 * @lc app=leetcode.cn id=98 lang=typescript
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

function isValidBST(root: TreeNode | null): boolean {
  let max = -Infinity

  function recur(root: TreeNode | null): boolean {
    //* 终止条件.遍历到左右叶子节点
    if (!root) return true
    //* 递归左子树
    let leftVaild = recur(root.left)
    //* 利用中序遍历的特点比较
    if (max < root.val) {
      max = root.val
    } else {
      return false
    }
    //* 递归右子树
    let rightVaild = recur(root.right)
    return leftVaild && rightVaild
  }
  return recur(root)
  // function sequence(root: TreeNode | null): number[] {
  //   if (!root) return
  //   sequence(root.left)
  //   res.push(root.val)
  //   sequence(root.right)
  //   return res
  // }
  // const res: number[] = []
  // sequence(root)
  // for (let i = 0; i < res.length - 1; i++) {
  //   if (res[i] >= res[i + 1]) return false
  // }
  // return true
}
// @lc code=end
