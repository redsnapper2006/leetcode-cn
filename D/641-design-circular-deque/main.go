package main

import "fmt"

// MyCircularDeque comments
type MyCircularDeque struct {
	Buf      []int
	Capacity int
}

// Constructor : Initialize your data structure here. Set the size of the deque to be k.
func Constructor(k int) MyCircularDeque {
	return MyCircularDeque{Buf: []int{}, Capacity: k}
}

// InsertFront Adds an item at the front of Deque. Return true if the operation is successful.
func (p *MyCircularDeque) InsertFront(value int) bool {
	if len(p.Buf) == p.Capacity {
		return false
	}
	p.Buf = append([]int{value}, p.Buf...)
	return true
}

// InsertLast Adds an item at the rear of Deque. Return true if the operation is successful.
func (p *MyCircularDeque) InsertLast(value int) bool {
	if len(p.Buf) == p.Capacity {
		return false
	}
	p.Buf = append(p.Buf, value)
	return true
}

// DeleteFront Deletes an item from the front of Deque. Return true if the operation is successful.
func (p *MyCircularDeque) DeleteFront() bool {
	if len(p.Buf) == 0 {
		return false
	}
	p.Buf = p.Buf[1:]
	return true
}

// DeleteLast Deletes an item from the rear of Deque. Return true if the operation is successful.
func (p *MyCircularDeque) DeleteLast() bool {
	if len(p.Buf) == 0 {
		return false
	}
	p.Buf = p.Buf[0 : len(p.Buf)-1]
	return true
}

// GetFront  Get the front item from the deque.
func (p *MyCircularDeque) GetFront() int {
	if len(p.Buf) == 0 {
		return -1
	}
	return p.Buf[0]
}

// GetRear Get the last item from the deque.
func (p *MyCircularDeque) GetRear() int {
	if len(p.Buf) == 0 {
		return -1
	}
	return p.Buf[len(p.Buf)-1]
}

// IsEmpty Checks whether the circular deque is empty or not.
func (p *MyCircularDeque) IsEmpty() bool {
	return len(p.Buf) == 0
}

// IsFull Checks whether the circular deque is full or not.
func (p *MyCircularDeque) IsFull() bool {
	return len(p.Buf) == p.Capacity
}

func main() {
	fmt.Println("a")
}
