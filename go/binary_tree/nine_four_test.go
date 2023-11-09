package binarytree

/*
 * @lc app=leetcode.cn id=94 lang=golang
 *
 * [94] 二叉树的中序遍历
 */

// @lc code=start
/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func inorder_traversal(root *TreeNode, res *[]int) {
	if root == nil {
		return
	}
	inorder_traversal(root.Left, res)
	*res = append(*res, root.Val)
	inorder_traversal(root.Right, res)
}
func inorderTraversal(root *TreeNode) []int {
	res := []int{}
	inorder_traversal(root, &res)
	return res
}

// @lc code=end
