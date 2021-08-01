package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

type CBTInserter struct {
	Level int
	Count int
	H     *TreeNode
}

func Constructor(root *TreeNode) CBTInserter {
	buf := []*TreeNode{root}
	H := CBTInserter{Level: 0, Count: 0, H: nil}
	for len(buf) > 0 {
		t := []*TreeNode{}
		for _, n := range buf {
			H.Insert(n.Val)
			if n.Left != nil {
				t = append(t, n.Left)
			}
			if n.Right != nil {
				t = append(t, n.Right)
			}
		}
		buf = t
	}
	return H
}

func (this *CBTInserter) Insert(v int) int {
	if this.Count == 0 {
		this.H = &TreeNode{Val: v}
		this.Count = 1
		this.Level = 1
		return -1
	}
	this.Count++
	if this.Count >= (1 << this.Level) {
		this.Level++
	}
	diff := this.Count - (1 << (this.Level - 1))
	p := this.H
	l := this.Level - 2
	for l > 0 {
		if diff&(1<<l) > 0 {
			p = p.Right
		} else {
			p = p.Left
		}
		l--
	}
	pv := p.Val
	if diff&1 > 0 {
		p.Right = &TreeNode{Val: v}
	} else {
		p.Left = &TreeNode{Val: v}
	}
	return pv
}

func (this *CBTInserter) Get_root() *TreeNode {
	return this.H
}

func main() {
	fmt.Println()
}
