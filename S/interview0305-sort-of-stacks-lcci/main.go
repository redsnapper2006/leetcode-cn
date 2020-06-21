package main

import "fmt"

type SortedStack struct {
	Stack []int
}

func Constructor() SortedStack {
	return SortedStack{Stack: []int{}}
}

func (this *SortedStack) Push(val int) {
	buf := []int{}
	for len(this.Stack) > 0 && this.Stack[len(this.Stack)-1] < val {
		buf = append(buf, this.Stack[len(this.Stack)-1])
		this.Stack = this.Stack[0 : len(this.Stack)-1]
	}
	this.Stack = append(this.Stack, val)
	for i := len(buf) - 1; i >= 0; i-- {
		this.Stack = append(this.Stack, buf[i])
	}
}

func (this *SortedStack) Pop() {
	if len(this.Stack) == 0 {
		return
	}
	this.Stack = this.Stack[0 : len(this.Stack)-1]
}

func (this *SortedStack) Peek() int {
	if len(this.Stack) == 0 {
		return -1
	}
	r := this.Stack[len(this.Stack)-1]
	return r
}

func (this *SortedStack) IsEmpty() bool {
	return len(this.Stack) == 0
}

func main() {
	fmt.Println("a")
}
