package main

import (
	"container/heap"
	"fmt"
)

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

func magicTower(nums []int) int {
	minh := &MinHeap{}
	heap.Init(minh)

	buf := []int{}
	blood := 1
	ret := 0
	for i := 0; i < len(nums); i++ {
		if nums[i] < 0 {
			heap.Push(minh, nums[i])
		}
		blood += nums[i]
		if blood <= 0 {
			for blood <= 0 && len(*minh) > 0 {
				max := heap.Pop(minh).(int)
				buf = append(buf, max)
				blood -= max
				ret++
				// fmt.Println(blood, max,i)
			}
			if blood <= 0 {
				return -1
			}
		}
	}
	for i := 0; i < len(buf); i++ {
		blood += buf[i]
		if blood <= 0 {
			return -1
		}
	}
	return ret
}

func main() {
	fmt.Println()
}
