package main

import "fmt"

type StackOfPlates struct {
	Capacity int
	StackArr []*[]int
}

func Constructor(cap int) StackOfPlates {
	return StackOfPlates{Capacity: cap, StackArr: []*[]int{}}
}

func (this *StackOfPlates) Push(val int) {
	if this.Capacity == 0 {
		return
	}
	if len(this.StackArr) == 0 || len(*this.StackArr[len(this.StackArr)-1]) == this.Capacity {
		this.StackArr = append(this.StackArr, &[]int{val})
	} else {
		offset := len(this.StackArr) - 1
		*this.StackArr[offset] = append(*this.StackArr[offset], val)
	}
}

func (this *StackOfPlates) Pop() int {
	if len(this.StackArr) == 0 || this.Capacity == 0 {
		return -1
	}
	offset := len(this.StackArr) - 1
	stack := this.StackArr[offset]
	r := (*stack)[len(*stack)-1]
	*stack = (*stack)[0 : len(*stack)-1]
	if len(*stack) == 0 {
		this.StackArr = this.StackArr[0:offset]
	}
	return r
}

func (this *StackOfPlates) PopAt(index int) int {
	if len(this.StackArr) == 0 || len(this.StackArr) <= index || this.Capacity == 0 {
		return -1
	}

	stack := this.StackArr[index]
	r := (*stack)[len(*stack)-1]
	*stack = (*stack)[0 : len(*stack)-1]
	if len(*stack) == 0 {
		this.StackArr = append(this.StackArr[0:index], this.StackArr[index+1:]...)
	}
	return r
}

func main() {
	o := Constructor(2)
	o.Push(1)
	o.Push(2)
	o.Push(3)
	fmt.Println(o.PopAt(0))
	fmt.Println(o.PopAt(0))
	fmt.Println(o.PopAt(0))
}
