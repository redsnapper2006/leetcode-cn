package main

import "fmt"

type Node struct {
	Val   int
	Prev  *Node
	Next  *Node
	Child *Node
}

func flatten(root *Node) *Node {

	var recur func(root *Node)
	recur = func(root *Node) {
		if root == nil {
			return
		}
		orgNext := root.Next
		if root.Child != nil {
			recur(root.Child)

			root.Child.Prev = root
			root.Next = root.Child
			p := root.Child
			for p.Next != nil {
				p = p.Next
			}

			p.Next = orgNext
			if orgNext != nil {
				orgNext.Prev = p
			}
			root.Child = nil
		}
		recur(orgNext)
	}
	recur(root)
	return root
}

func main() {
	fmt.Println()
}
