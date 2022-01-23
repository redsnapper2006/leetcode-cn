package main

import (
	"fmt"
	"sort"
)

type IntArrSlice [][]int

func (p IntArrSlice) Len() int {
	return len(p)
}

func (p IntArrSlice) Swap(i, j int) {
	p[i], p[j] = p[j], p[i]
}

func (p IntArrSlice) Less(i, j int) bool {
	if p[i][0] != p[j][0] {
		return p[i][0] < p[j][0]
	}

	return p[i][1] > p[j][1]
}

func maximumBeauty(items [][]int, queries []int) []int {
	sort.Sort(IntArrSlice(items))

	base := 0
	max := -1
	buf := [][]int{}
	for i := 0; i < len(items); i++ {
		if items[i][0] != base {
			base = items[i][0]
			if max < items[i][1] {
				max = items[i][1]
			}
			buf = append(buf, []int{items[i][0], max})
		}
	}

	ret := []int{}
	for i := 0; i < len(queries); i++ {
		s, e := 0, len(buf)-1
		for s <= e {
			m := s + (e-s)/2
			if buf[m][0] > queries[i] {
				e = m - 1
			} else {
				s = m + 1
			}
		}
		if s == 0 {
			ret = append(ret, -1)
		} else {
			ret = append(ret, buf[s-1][1])
		}
	}
	return ret
}

func main() {
	fmt.Println()
}
