package main

type MyCircularQueue struct {
	Q []int
	C int
	O int
}

/** Initialize your data structure here. Set the size of the queue to be k. */
func Constructor(k int) MyCircularQueue {
	return MyCircularQueue{Q: make([]int, k), C: 0, O: 0}
}

/** Insert an element into the circular queue. Return true if the operation is successful. */
func (this *MyCircularQueue) EnQueue(value int) bool {
	if this.C < len(this.Q) {
		this.Q[(this.C+this.O)%len(this.Q)] = value
		this.C++
		return true
	}
	return false
}

/** Delete an element from the circular queue. Return true if the operation is successful. */
func (this *MyCircularQueue) DeQueue() bool {
	if this.C > 0 {
		this.C--
		this.O++
		this.O %= len(this.Q)
		return true
	}
	return false
}

/** Get the front item from the queue. */
func (this *MyCircularQueue) Front() int {
	if this.C > 0 {
		return this.Q[this.O]
	}
	return -1
}

/** Get the last item from the queue. */
func (this *MyCircularQueue) Rear() int {
	if this.C > 0 {
		return this.Q[(this.C+this.O-1)%len(this.Q)]
	}
	return -1
}

/** Checks whether the circular queue is empty or not. */
func (this *MyCircularQueue) IsEmpty() bool {
	return this.C == 0
}

/** Checks whether the circular queue is full or not. */
func (this *MyCircularQueue) IsFull() bool {
	return this.C == len(this.Q)
}

func main() {
	fmt.Println("a")
}
