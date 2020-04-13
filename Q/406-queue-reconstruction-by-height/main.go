package main

import (
	"fmt"
	"sort"
)

type QueSlice [][]int

func (p QueSlice) Len() int {
	return len(p)
}

func (p QueSlice) Swap(i, j int) {
	p[i], p[j] = p[j], p[i]
}

func (p QueSlice) Less(i, j int) bool {
	if p[i][1] == p[j][1] {
		return p[i][0] < p[j][0]
	}

	return p[i][1] < p[j][1]
}

type Que2Slice [][]int

func (p Que2Slice) Len() int {
	return len(p)
}

func (p Que2Slice) Swap(i, j int) {
	p[i], p[j] = p[j], p[i]
}

func (p Que2Slice) Less(i, j int) bool {
	if p[i][0] == p[j][0] {
		return p[i][1] < p[j][1]
	}
	return p[i][0] > p[j][0]
}

func reconstructQueue(people [][]int) [][]int {
	sort.Sort(Que2Slice(people))
	for i := 0; i < len(people); i++ {
		if people[i][1] < i {
			t := people[i]
			copy(people[t[1]+1:i+1], people[t[1]:i])
			people[t[1]] = t
		}
	}
	return people
}

func reconstructQueueV2(people [][]int) [][]int {
	w := make([][]int, len(people))
	for i := 0; i < len(w); i++ {
		w[i] = []int{people[i][0], people[i][1], people[i][1]}
	}

	buf := make([][]int, len(w))
	idx := 0
	for len(w) > 0 {
		sort.Sort(QueSlice(w))
		buf[idx] = []int{w[0][0], w[0][2]}
		for i := 1; i < len(w); i++ {
			if w[i][0] <= w[0][0] {
				w[i][1]--
			}
		}
		w = w[1:]
		idx++
	}
	return buf
}

func main() {
	fmt.Println(reconstructQueue([][]int{
		{7, 0}, {4, 4}, {7, 1}, {5, 0}, {6, 1}, {5, 2},
	}))
}
