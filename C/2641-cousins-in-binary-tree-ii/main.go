package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func replaceValueInTree(root *TreeNode) *TreeNode {
	buf := [][]int{}

	var recur1 func(node *TreeNode, level int)
	recur1 = func(node *TreeNode, level int) {
		if len(buf) < level+1 {
			buf = append(buf, []int{})
		}

		if node == nil {
			buf[level] = append(buf[level], 0)
			return
		}

		buf[level] = append(buf[level], node.Val)

		recur1(node.Left, level+1)
		recur1(node.Right, level+1)
	}

	recur1(root, 0)

	sums := []int{}
	for _, row := range buf {
		sum := 0
		for _, v := range row {
			sum += v
		}
		sums = append(sums, sum)
	}

	var recur2 func(node *TreeNode, level int, sibling int)
	recur2 = func(node *TreeNode, level int, sibling int) {
		if node == nil {
			return
		}

		node.Val = sums[level] - sibling - node.Val

		right := 0
		left := 0
		if node.Right != nil {
			right = node.Right.Val
		}
		if node.Left != nil {
			left = node.Left.Val
		}
		recur2(node.Left, level+1, right)
		recur2(node.Right, level+1, left)
	}
	recur2(root, 0, 0)
	return root
}
