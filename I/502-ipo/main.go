package main

import (
	"container/heap"
	"fmt"
	"sort"
)

type CPArr [][]int

func (p CPArr) Len() int {
	return len(p)
}

func (p CPArr) Swap(i, j int) {
	p[i], p[j] = p[j], p[i]
}

func (p CPArr) Less(i, j int) bool {
	// if p[i][0] == p[j][0] {
	// 	return p[i][1] < p[j][1]
	// }

	return p[i][0] < p[j][0]
}

type MaxHeap []int

func (p MaxHeap) Len() int {
	return len(p)
}

func (p MaxHeap) Swap(i, j int) {
	p[i], p[j] = p[j], p[i]
}

func (p MaxHeap) Less(i, j int) bool {
	return p[i] > p[j]
}

func (p *MaxHeap) Push(c interface{}) {
	*p = append(*p, c.(int))
}
func (p *MaxHeap) Pop() interface{} {
	n := len(*p)
	v := (*p)[n-1]
	*p = (*p)[0 : n-1]
	return v
}

func findMaximizedCapital(k int, w int, profits []int, capital []int) int {
	cparr := make([][]int, len(profits))
	for i := 0; i < len(profits); i++ {
		cparr[i] = []int{capital[i], profits[i]}
	}
	sort.Sort(CPArr(cparr))

	maxh := &MaxHeap{}
	heap.Init(maxh)
	idx := 0
	base := w
	ret := w
	for i := 0; i < k; i++ {
		for idx < len(cparr) && cparr[idx][0] <= base {
			heap.Push(maxh, cparr[idx][1])
			idx++
		}
		// fmt.Println(*maxh)
		if len(*maxh) > 0 {
			v := heap.Pop(maxh).(int)
			ret += v
			base += v
		}
	}
	return ret
}

func main() {
	fmt.Println()
}
