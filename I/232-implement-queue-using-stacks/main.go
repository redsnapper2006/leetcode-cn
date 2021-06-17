package main

import (
	"fmt"
)

type MyQueue struct {
	Q []int
}

/** Initialize your data structure here. */
func Constructor() MyQueue {
	return MyQueue{Q: []int{}}
}

/** Push element x to the back of queue. */
func (this *MyQueue) Push(x int) {
	this.Q = append(this.Q, x)
}

/** Removes the element from in front of queue and returns that element. */
func (this *MyQueue) Pop() int {
	r := this.Q[0]
	this.Q = this.Q[1:]
	return r
}

/** Get the front element. */
func (this *MyQueue) Peek() int {
	return this.Q[0]
}

/** Returns whether the queue is empty. */
func (this *MyQueue) Empty() bool {
	return len(this.Q) == 0
}

func main() {
	fmt.Println("a")
}
