package main

import "fmt"

type MyStack struct {
	Q []int
}

/** Initialize your data structure here. */
func Constructor() MyStack {
	return MyStack{Q: []int{}}
}

/** Push element x onto stack. */
func (this *MyStack) Push(x int) {
	this.Q = append(this.Q, x)
}

/** Removes the element on top of the stack and returns that element. */
func (this *MyStack) Pop() int {
	v := this.Q[len(this.Q)-1]
	this.Q = this.Q[0 : len(this.Q)-1]
	return v
}

/** Get the top element. */
func (this *MyStack) Top() int {
	return this.Q[len(this.Q)-1]
}

/** Returns whether the stack is empty. */
func (this *MyStack) Empty() bool {
	return len(this.Q) == 0
}

func main() {
	fmt.Println("a")
}
