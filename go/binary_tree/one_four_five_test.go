package binarytree

/*
 * @lc app=leetcode.cn id=145 lang=golang
 *
 * [145] 二叉树的后序遍历
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
func traversal_postorder(root *TreeNode, res *[]int) {
	if root == nil {
		return
	}
	traversal_postorder(root.Left, res)
	traversal_postorder(root.Right, res)
	*res = append(*res, root.Val)
}

func postorderTraversal(root *TreeNode) []int {
	res := []int{}
	traversal_postorder(root, &res)
	return res
}

// @lc code=end
