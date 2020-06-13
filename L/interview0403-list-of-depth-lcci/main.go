package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

type ListNode struct {
	Val  int
	Next *ListNode
}

func listOfDepth(tree *TreeNode) []*ListNode {
	if tree == nil {
		return nil
	}

	var b []*ListNode
	b = append(b, &ListNode{Val: tree.Val})

	bb := []*TreeNode{tree}
	for len(bb) > 0 {
		var tt []*TreeNode
		H := ListNode{Val: 0}
		p := &H
		for i := 0; i < len(bb); i++ {
			if bb[i].Left != nil {
				tt = append(tt, bb[i].Left)
				p.Next = &ListNode{Val: bb[i].Left.Val}
				p = p.Next
			}
			if bb[i].Right != nil {
				tt = append(tt, bb[i].Right)
				p.Next = &ListNode{Val: bb[i].Right.Val}
				p = p.Next
			}
		}
		if len(tt) > 0 {
			b = append(b, H.Next)
		}
		bb = tt
	}
	return b
}

func main() {
	fmt.Println("a")
}
