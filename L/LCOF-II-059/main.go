package main

import "container/heap"

type MinHeap []int

func (p MinHeap) Len() int {
	return len(p)
}

func (p MinHeap) Swap(i, j int) {
	p[i], p[j] = p[j], p[i]
}

func (p MinHeap) Less(i, j int) bool {
	return p[i] < p[j]
}

func (p *MinHeap) Push(c interface{}) {
	*p = append(*p, c.(int))
}

func (p *MinHeap) Pop() interface{} {
	n := len(*p)
	v := (*p)[n-1]
	*p = (*p)[0 : n-1]
	return v
}

type KthLargest struct {
	MH *MinHeap
	K  int
}

func Constructor(k int, nums []int) KthLargest {
	mh := &MinHeap{}
	heap.Init(mh)
	for _, v := range nums {
		heap.Push(mh, v)
	}
	if len(nums) > k {
		for i := 0; i < len(nums)-k; i++ {
			heap.Pop(mh)
		}
	}
	return KthLargest{MH: mh, K: k}
}

func (this *KthLargest) Add(val int) int {
	if len(*this.MH) < this.K {
		heap.Push(this.MH, val)
		v := heap.Pop(this.MH).(int)
		heap.Push(this.MH, v)
		return v
	}
	v := heap.Pop(this.MH).(int)
	if v <= val {
		heap.Push(this.MH, val)
		v = heap.Pop(this.MH).(int)
	}

	heap.Push(this.MH, v)
	return v
}
