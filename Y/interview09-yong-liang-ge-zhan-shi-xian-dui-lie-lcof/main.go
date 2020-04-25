package main

import (
	"fmt"
)

type CQueue struct {
	T    []int
	H    []int
	TorH byte
}

func Constructor() CQueue {
	return CQueue{T: []int{}, H: []int{}, TorH: 'T'}
}

func Push(Stack *[]int, value int) {
	*Stack = append(*Stack, value)
}

func Pop(Stack *[]int) int {
	v := (*Stack)[len(*Stack)-1]
	*Stack = (*Stack)[0 : len(*Stack)-1]
	return v
}

func IsEmpty(Stack *[]int) bool {
	return len(*Stack) == 0
}

func (this *CQueue) AppendTail(value int) {
	if this.TorH == 'H' {
		for !IsEmpty(&this.H) {
			Push(&this.T, Pop(&this.H))
		}
		// this.H = nil
	}
	this.T = append(this.T, value)
	this.TorH = 'T'
}

func (this *CQueue) DeleteHead() int {
	if this.TorH == 'T' {
		for !IsEmpty(&this.T) {
			Push(&this.H, Pop(&this.T))
		}
	}
	this.TorH = 'H'
	if IsEmpty(&this.H) {
		return -1
	}
	return Pop(&this.H)
}

func main() {
	o := Constructor()
	fmt.Println("tail")
	o.AppendTail(3)
	fmt.Println("hesd")
	fmt.Println(o.DeleteHead())
	fmt.Println(o.DeleteHead())

}
