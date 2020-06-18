package main

import "fmt"

type MinStack struct {
	Stack  []int
	Min    []int
	Offset int
}

/** initialize your data structure here. */
func Constructor() MinStack {
	return MinStack{Stack: make([]int, 10000), Min: make([]int, 10000), Offset: 0}
}

func (this *MinStack) Push(x int) {
	this.Stack[this.Offset] = x
	if this.Offset == 0 {
		this.Min[this.Offset] = x
	} else {
		if this.Min[this.Offset-1] > x {
			this.Min[this.Offset] = x
		} else {
			this.Min[this.Offset] = this.Min[this.Offset-1]
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

func (this *MinStack) GetMin() int {
	return this.Min[this.Offset-1]
}

func main() {
	fmt.Println("a")
}
