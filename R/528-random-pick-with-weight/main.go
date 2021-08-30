package main

import (
	"math/rand"
	"sort"
)

type Solution struct {
	Sum   []int
	Total int
}

func Constructor(w []int) Solution {
	sum := []int{}
	s := 0
	for _, v := range w {
		s += v
		sum = append(sum, s)
	}
	return Solution{Sum: sum, Total: s}
}

func (this *Solution) PickIndex() int {
	r := rand.Intn(this.Total) + 1
	return sort.SearchInts(this.Sum, r)
}

func main() {
	s := Constructor([]int{3, 4, 1, 7})
	s.PickIndex()
	s.PickIndex()
	s.PickIndex()
	s.PickIndex()
	s.PickIndex()
	s.PickIndex()
	s.PickIndex()
	s.PickIndex()
	s.PickIndex()
	s.PickIndex()
	s.PickIndex()
	s.PickIndex()
	s.PickIndex()
	s.PickIndex()
	s.PickIndex()
	s.PickIndex()
	s.PickIndex()
	s.PickIndex()
	s.PickIndex()
	s.PickIndex()
	s.PickIndex()
	s.PickIndex()
	s.PickIndex()
	s.PickIndex()
	s.PickIndex()
	s.PickIndex()
	s.PickIndex()
	s.PickIndex()
	s.PickIndex()
	s.PickIndex()
	s.PickIndex()
	s.PickIndex()
	s.PickIndex()
	s.PickIndex()
	s.PickIndex()
	s.PickIndex()
}
