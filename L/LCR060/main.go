package main

import (
	"container/heap"
)

type MinHeap [][]int

func (p MinHeap) Len() int {
	return len(p)
}

func (p MinHeap) Swap(i, j int) {
	p[i], p[j] = p[j], p[i]
}

func (p MinHeap) Less(i, j int) bool {
	return p[i][0] > p[j][0]
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

func topKFrequent(nums []int, k int) []int {
	mh := &MinHeap{}
	heap.Init(mh)

	m := map[int]int{}
	for _, v := range nums {
		m[v]++
		heap.Push(mh, []int{m[v], v})
	}

	cnt := 0
	m1 := map[int]int{}
	for cnt < k {
		c := heap.Pop(mh).([]int)
		// fmt.Println(c)
		_, ok := m1[c[1]]
		if !ok {
			m1[c[1]]++
			cnt++
		}
	}

	ret := []int{}
	for k := range m1 {
		ret = append(ret, k)
	}

	return ret
}
