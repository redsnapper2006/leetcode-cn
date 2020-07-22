package main

import "fmt"

type MinStack struct {
	Stack  []int
	MinS   []int
	Offset int
}

/** initialize your data structure here. */
func Constructor() MinStack {
	return MinStack{Stack: make([]int, 1000), MinS: make([]int, 1000), Offset: 0}
}

func (this *MinStack) Push(x int) {
	this.Stack[this.Offset] = x
	if this.Offset == 0 {
		this.MinS[this.Offset] = x
	} else {
		if this.MinS[this.Offset-1] > x {
			this.MinS[this.Offset] = x
		} else {
			this.MinS[this.Offset] = this.MinS[this.Offset-1]
		}
	}
	this.Offset++
}

func (this *MinStack) Pop() {
	this.Offset--
}

func (this *MinStack) Top() int {
	return this.Stack[this.Offset-1]
}

func (this *MinStack) Min() int {
	return this.MinS[this.Offset-1]
}

func main() {
	fmt.Println("a")
}
