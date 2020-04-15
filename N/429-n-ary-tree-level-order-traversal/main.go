package main

type Node struct {
	Val      int
	Children []*Node
}

func levelOrder(root *Node) [][]int {
	if root == nil {
		return [][]int{}
	}
	var buf [][]int
	buf = append(buf, []int{root.Val})
	stack := []*Node{root}

	for len(stack) > 0 {
		var t []*Node
		var b []int
		for i := 0; i < len(stack); i++ {
			n := stack[i]
			for j := 0; j < len(n.Children); j++ {
				b = append(b, n.Children[j].Val)
				t = append(t, n.Children[j])
			}
		}
		if len(b) > 0 {
			buf = append(buf, b)
		}
		stack = t
	}
	return buf
}

func main() {
	fmt.Println("a")
}
