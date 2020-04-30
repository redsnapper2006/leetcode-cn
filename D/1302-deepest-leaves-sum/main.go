package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func deepestLeavesSum(root *TreeNode) int {
	if root == nil {
		return 0
	}

	stack := []*TreeNode{root}
	for len(stack) > 0 {
		var t []*TreeNode
		for i := 0; i < len(stack); i++ {
			if stack[i].Left != nil {
				t = append(t, stack[i].Left)
			}
			if stack[i].Right != nil {
				t = append(t, stack[i].Right)
			}
		}
		if len(t) == 0 {
			break
		} else {
			stack = t
		}
	}

	sum := 0
	for i := 0; i < len(stack); i++ {
		sum += stack[i].Val
	}

	return sum
}

func main() {

	fmt.Println("a")
}
