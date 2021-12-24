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
	return p[i][1] < p[j][1]
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

func eatenApples(apples []int, days []int) int {
	h := &MinHeap{}
	heap.Init(h)

	cnt := 0
	for i := 0; i < len(apples); i++ {
		if apples[i] > 0 {
			heap.Push(h, []int{apples[i], i + days[i]})
		}
		for len(*h) > 0 {
			candi := heap.Pop(h).([]int)
			if candi[1] > i {
				candi[0]--
				if candi[0] > 0 {
					heap.Push(h, candi)
				}
				cnt++
				break
			}
		}
	}

	day := len(apples)
	for len(*h) > 0 {
		candi := heap.Pop(h).([]int)
		if candi[1] > day {
			candi[0]--
			if candi[0] > 0 {
				heap.Push(h, candi)
			}
			cnt++
			day++
		}
	}
	return cnt
}

func main() {
	fmt.Println()
}
