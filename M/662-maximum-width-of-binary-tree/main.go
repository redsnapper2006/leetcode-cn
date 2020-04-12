package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func widthOfBinaryTree(root *TreeNode) int {
	if root == nil {
		return 0
	}
	max := 1
	stack := []*TreeNode{root}
	for {
		var t []*TreeNode

		for i := 0; i < len(stack); i++ {
			r := stack[i]
			if r != nil {
				t = append(t, r.Left, r.Right)
			} else {
				t = append(t, nil, nil)
			}
		}
		s := 0
		for s < len(t) {
			if t[s] != nil {
				break
			}
			s++
		}
		if s == len(t) {
			break
		}
		e := len(t) - 1
		for e >= 0 {
			if t[e] != nil {
				break
			}
			e--
		}
		if e == -1 {
			break
		}
		if e-s+1 > max {
			max = e - s + 1
		}
		stack = t[s : e+1]
	}
	return max
}

func main() {
	fmt.Println("a")
}
