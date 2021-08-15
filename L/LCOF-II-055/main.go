package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

type BSTIterator struct {
	Stack []*TreeNode
}

func Constructor(root *TreeNode) BSTIterator {
	p := root
	buf := []*TreeNode{}
	for p != nil {
		buf = append(buf, p)
		p = p.Left
	}
	return BSTIterator{Stack: buf}
}

func (this *BSTIterator) Next() int {
	p := this.Stack[len(this.Stack)-1]
	v := p.Val
	this.Stack = this.Stack[0 : len(this.Stack)-1]
	if p.Right != nil {
		p = p.Right
		for p != nil {
			this.Stack = append(this.Stack, p)
			p = p.Left
		}
	}
	return v
}

func (this *BSTIterator) HasNext() bool {
	return len(this.Stack) > 0
}

func main() {
	fmt.Println()
}
