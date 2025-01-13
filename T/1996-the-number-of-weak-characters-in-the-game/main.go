package main

import (
	"container/heap"
	"fmt"
	"sort"
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

type Arr [][]int

func (ss Arr) Len() int {
	return len(ss)
}

func (ss Arr) Swap(i, j int) {
	ss[i], ss[j] = ss[j], ss[i]
}

func (ss Arr) Less(i, j int) bool {
	if ss[i][0] == ss[j][0] {
		return ss[i][1] < ss[j][1]
	}
	return ss[i][0] > ss[j][0]
}

func numberOfWeakCharactersV2(properties [][]int) int {
	sort.Sort(Arr(properties))
	// fmt.Println(properties)
	ret := 0

	buf := &MinHeap{}
	heap.Init(buf)

	sbuf := []int{}
	for i := 0; i < len(properties); i++ {
		if i == 0 || properties[i][0] == properties[i-1][0] {
			sbuf = append(sbuf, properties[i][1])
		} else {
			for _, s := range sbuf {
				heap.Push(buf, s)
			}
			sbuf = []int{properties[i][1]}
		}

		def := properties[i][1]
		if len(*buf) > 0 {
			t := heap.Pop(buf)
			v := t.(int)
			isRollback := true
			for v < def {
				ret++
				if len(*buf) == 0 {
					isRollback = false
					break
				}
				t = heap.Pop(buf)
				v = t.(int)
			}
			if isRollback {
				heap.Push(buf, v)
			}
		}
	}

	return ret
}

func numberOfWeakCharacters(properties [][]int) int {
	sort.Sort(Arr(properties))
	// fmt.Println(properties)
	max := 0
	ret := 0
	for i := 0; i < len(properties); i++ {
		if properties[i][1] >= max {
			max = properties[i][1]
		} else {
			ret++
		}
	}
	return ret
}

func main() {
	fmt.Println(numberOfWeakCharacters([][]int{{1, 1}, {2, 1}, {2, 2}, {1, 2}}))
	fmt.Println(numberOfWeakCharacters([][]int{{1, 5}, {10, 4}, {4, 3}}))
	fmt.Println(numberOfWeakCharacters([][]int{{7, 7}, {1, 2}, {9, 7}, {7, 3}, {3, 10}, {9, 8}, {8, 10}, {4, 3}, {1, 5}, {1, 5}}))
}
