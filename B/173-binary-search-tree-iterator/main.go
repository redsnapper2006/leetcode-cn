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
	if root == nil {
		return BSTIterator{Stack: []*TreeNode{}}
	} else {
		p := root
		var s []*TreeNode
		for p != nil {
			s = append(s, p)
			p = p.Left
		}
		return BSTIterator{Stack: s}
	}
}

/** @return the next smallest number */
func (this *BSTIterator) Next() int {
	p := this.Stack[len(this.Stack)-1]
	this.Stack = this.Stack[0 : len(this.Stack)-1]
	if p.Right != nil {
		q := p.Right
		for q != nil {
			this.Stack = append(this.Stack, q)
			q = q.Left
		}
	}
	return p.Val
}

/** @return whether we have a next smallest number */
func (this *BSTIterator) HasNext() bool {
	return len(this.Stack) > 0
}

func main() {
	fmt.Println("a")
}
