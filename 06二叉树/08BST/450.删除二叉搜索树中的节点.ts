/**
 * /*
 *
 * [450] 删除二叉搜索树中的节点
 *
 * @format
 * @lc app=leetcode.cn id=450 lang=typescript
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

function deleteNode(root: TreeNode | null, key: number): TreeNode | null {
  if (!root) return null
  if (root.val > key) root.left = deleteNode(root.left, key)
  if (root.val < key) root.right = deleteNode(root.right, key)
  if (root.val === key) {
    //* key为叶子节点
    if (!root.left && !root.right) return null
    //* key节点下只连接一个节点
    if (!root.left) return root.right
    if (!root.right) return root.left
    //* key节点下连接两个节点
    let curNode = root.right
    //删除节点的右节点的左节点一直遍历到null
    while (curNode.left) curNode = curNode.left
    curNode.left = root.left
    return root.right
  }
  return root
}
// @lc code=end

export {}
