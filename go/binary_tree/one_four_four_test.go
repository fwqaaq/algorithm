package binarytree

import (
	"testing"
)

/*
 * @lc app=leetcode.cn id=144 lang=golang
 *
 * [144] 二叉树的前序遍历
 */

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

// @lc code=start
/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */

func traversal(root *TreeNode, res *[]int) {
	if root == nil {
		return
	}
	*res = append(*res, root.Val)
	traversal(root.Left, res)
	traversal(root.Right, res)
}

func preorderTraversal(root *TreeNode) []int {
	// res := []int{}
	// traversal(root, &res)
	// return res

	/// Iteration
	res := []int{}
	if root == nil {
		return res
	}
	stack := []*TreeNode{root}
	for len(stack) > 0 {
		node := stack[len(stack)-1]
		stack = stack[:len(stack)-1]
		res = append(res, node.Val)
		if node.Right != nil {
			stack = append(stack, node.Right)
		}
		if node.Left != nil {
			stack = append(stack, node.Left)
		}
	}
	return res
}

// @lc code=end

func TestPreorderTraversal(t *testing.T) {
	root := TreeNode{
		Val: 1,
		Left: &TreeNode{
			Val: 2,
			Left: &TreeNode{
				Val: 4,
			},
			Right: nil,
		},
		Right: &TreeNode{
			Val: 3,
		},
	}
	res := preorderTraversal(&root)
	t.Log("I'm in test", res)
}
