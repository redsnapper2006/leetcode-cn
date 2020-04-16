package main

type Node struct {
	Val      int
	Children []*Node
}

func maxDepth(root *Node) int {
	if root == nil {
		return 0
	}

	level := 0
	stack := []*Node{root}
	for len(stack) > 0 {
		level++
		var t []*Node
		for i := 0; i < len(stack); i++ {
			n := stack[i]
			for j := 0; j < len(n.Children); j++ {
				t = append(t, n.Children[j])
			}
		}
		stack = t
	}
	return level
}

func main() {
	fmt.Println("a")
}
