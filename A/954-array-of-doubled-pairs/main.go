package main

import (
	"fmt"
	"sort"
)

type AbsInt []int

func (p AbsInt) Len() int {
	return len(p)
}

func (p AbsInt) Swap(i, j int) {
	p[i], p[j] = p[j], p[i]
}

func (p AbsInt) Less(i, j int) bool {
	pi, pj := p[i], p[j]
	if pi < 0 {
		pi = -pi
	}
	if pj < 0 {
		pj = -pj
	}
	return pi < pj
}

func canReorderDoubled(arr []int) bool {
	m := map[int]int{}
	for _, v := range arr {
		m[v]++
	}

	buf := []int{}
	for k := range m {
		buf = append(buf, k)
	}

	sort.Sort(AbsInt(buf))
	for _, k := range buf {
		if m[k] == 0 {
			continue
		}
		b, t := k, 2*k

		if m[t] >= m[b] {
			m[t] -= m[b]
			m[b] = 0
		} else {
			return false
		}
	}

	for _, v := range m {
		if v > 0 {
			return false
		}
	}
	return true
}

func main() {

	fmt.Println(canReorderDoubled([]int{1, 2, 1, -8, 8, -4, 4, -4, 2, -2}))
	fmt.Println(canReorderDoubled([]int{-6, -3}))
}
