package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

type KthLargest struct {
	Head *TreeNode
	K    int
	C    int
}

func Constructor(k int, nums []int) KthLargest {
	t := TreeNode{Val: 1<<63 - 1, Left: nil, Right: nil}
	b := KthLargest{Head: &t, K: k, C: 0}

	for i := 0; i < len(nums); i++ {
		b.Add(nums[i])
	}
	return b
}

func (this *KthLargest) Add(val int) int {
	if this.C < this.K || this.Head.Val < val {
		if val <= this.Head.Val {
			t := TreeNode{Val: val, Left: nil, Right: this.Head}
			this.Head = &t
		} else {
			p := this.Head
			for p.Right != nil && p.Right.Val < val {
				p = p.Right
			}
			t := TreeNode{Val: val, Left: nil, Right: p.Right}
			p.Right = &t
			if this.C >= this.K {
				this.Head = this.Head.Right
			}
		}
		this.C++
	}
	return this.Head.Val
}

func (this *KthLargest) Print() {
	p := this.Head

	for p != nil {
		fmt.Println(p.Val)
		p = p.Right
	}
}
func main() {
	o := Constructor(3, []int{4, 5, 8, 2})
	fmt.Println("###")
	o.Print()
	o.Add(3) // returns 4
	o.Print()
	o.Add(5) // returns 5
	o.Print()
	o.Add(10) // returns 5
	o.Print()
	o.Add(9) // returns 8
	o.Print()
	o.Add(9) // returns 8
	o.Add(4) // returns 8
	o.Print()
}
