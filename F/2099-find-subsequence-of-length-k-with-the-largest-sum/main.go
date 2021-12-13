package main

import (
	"container/heap"
	"fmt"
)

type MinHeap [][]int

func (p MinHeap) Len() int {
	return len(p)
}

func (p MinHeap) Swap(i, j int) {
	p[i], p[j] = p[j], p[i]
}

func (p MinHeap) Less(i, j int) bool {
	return p[i][0] < p[j][0]
}

func (p *MinHeap) Push(c interface{}) {
	*p = append(*p, c.([]int))
}
func (p *MinHeap) Pop() interface{} {
	n := len(*p)
	v := (*p)[n-1]
	*p = (*p)[0 : n-1]
	return v
}

func maxSubsequence(nums []int, k int) []int {
	h := &MinHeap{}
	heap.Init(h)
	for i := 0; i < k; i++ {
		heap.Push(h, []int{nums[i], i})
	}

	for i := k; i < len(nums); i++ {
		heap.Push(h, []int{nums[i], i})
		heap.Pop(h)
	}
	// fmt.Println(h)
	idxM := map[int]int{}
	for h.Len() > 0 {
		v := heap.Pop(h).([]int)
		idxM[v[1]]++
	}

	// fmt.Println(idxM)
	ret := []int{}
	for i := 0; i < len(nums); i++ {
		_, ok := idxM[i]
		if ok {
			ret = append(ret, nums[i])
		}
	}
	return ret
}

func main() {
	fmt.Println()
}
