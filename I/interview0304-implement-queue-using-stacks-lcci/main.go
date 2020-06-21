package main

import "fmt"

type MyQueue struct {
	PushStack []int
	PopStack  []int
}

/** Initialize your data structure here. */
func Constructor() MyQueue {
	return MyQueue{PushStack: []int{}, PopStack: []int{}}
}

/** Push element x to the back of queue. */
func (this *MyQueue) Push(x int) {
	this.PushStack = append(this.PushStack, x)
}

/** Removes the element from in front of queue and returns that element. */
func (this *MyQueue) Pop() int {
	if len(this.PopStack) == 0 {
		for i := len(this.PushStack) - 1; i >= 0; i-- {
			this.PopStack = append(this.PopStack, this.PushStack[i])
		}
		this.PushStack = []int{}
	}

	r := this.PopStack[len(this.PopStack)-1]
	this.PopStack = this.PopStack[0 : len(this.PopStack)-1]
	return r
}

/** Get the front element. */
func (this *MyQueue) Peek() int {
	if len(this.PopStack) == 0 {
		for i := len(this.PushStack) - 1; i >= 0; i-- {
			this.PopStack = append(this.PopStack, this.PushStack[i])
		}
		this.PushStack = []int{}
	}

	return this.PopStack[len(this.PopStack)-1]
}

/** Returns whether the queue is empty. */
func (this *MyQueue) Empty() bool {
	return len(this.PushStack) == 0 && len(this.PopStack) == 0
}

func main() {
	fmt.Println("a")
}
