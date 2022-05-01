/**
 * /*
 *
 * [513] 找树左下角的值
 *
 * @format
 * @lc app=leetcode.cn id=513 lang=typescript
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

function findBottomLeftValue(root: TreeNode | null): number {
  // if (!root) return 0
  // let leftValue = 0
  // let queue: TreeNode[] = [root]
  // while (queue.length) {
  //   let len = queue.length
  //   for (let i = 0; i < len; i++) {
  //     let curNode = queue.shift()
  //     //* 当前节点最左边的节点取值
  //     if (i === 0) {
  //       leftValue = curNode!.val
  //     }
  //     //* 从当前节点的左右节点放入队列
  //     if (curNode!.left) queue.push(curNode!.left)
  //     if (curNode!.right) queue.push(curNode!.right)
  //   }
  // }
  // return leftValue
  if (!root) return 0
  function recur(root: TreeNode, maxDepth: number) {
    if (root.left === null && root.right === null) {
      //* 只有当节点深度大于等于当前深度时,才会更新最大深度
      if (curDepth < maxDepth) {
        // 更新当前深度,为最大深度
        curDepth = maxDepth
        leftValue = root.val
      }
      return
    }
    //* 当前节点不是叶子节点
    //* 回溯.从左子树开始,拿到左子树的最大深度的左节点
    if (root.left) recur(root.left, maxDepth + 1)
    if (root.right) recur(root.right, maxDepth + 1)
  }

  let curDepth = 0
  let leftValue = 0
  recur(root, 1)
  return leftValue
}
// @lc code=end

export {}
