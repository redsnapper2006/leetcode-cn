package main

import "fmt"

type TripleInOne struct {
	Stack    []int
	Capacity int
	Offset   []int
}

func Constructor(stackSize int) TripleInOne {
	return TripleInOne{Stack: make([]int, stackSize*3), Capacity: stackSize, Offset: make([]int, 3)}
}

func (this *TripleInOne) Push(stackNum int, value int) {
	if this.Offset[stackNum] == this.Capacity {
		return
	}

	this.Stack[stackNum*this.Capacity+this.Offset[stackNum]] = value
	this.Offset[stackNum]++
}

func (this *TripleInOne) Pop(stackNum int) int {
	if this.Offset[stackNum] == 0 {
		return -1
	}
	this.Offset[stackNum]--
	return this.Stack[stackNum*this.Capacity+this.Offset[stackNum]]
}

func (this *TripleInOne) Peek(stackNum int) int {
	if this.Offset[stackNum] == 0 {
		return -1
	}
	return this.Stack[stackNum*this.Capacity+this.Offset[stackNum]-1]
}

func (this *TripleInOne) IsEmpty(stackNum int) bool {
	return this.Offset[stackNum] == 0
}

func main() {
	fmt.Println("a")
}
