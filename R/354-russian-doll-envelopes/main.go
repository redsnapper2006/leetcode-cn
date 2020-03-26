package main

import (
	"fmt"
	"sort"
)

type IntArr [][]int

func (p IntArr) Len() int {
	return len(p)
}

func (p IntArr) Swap(i, j int) {
	p[i], p[j] = p[j], p[i]
}

func (p IntArr) Less(i, j int) bool {
	w1, h1, w2, h2 := p[i][0], p[i][1], p[j][0], p[j][1]
	if w1 != w2 {
		return w1 < w2
	} else if h1 != h2 {
		return h1 < h2
	}
	return 1 == 1
}

func maxEnvelopes(envelopes [][]int) int {
	if len(envelopes) <= 1 {
		return len(envelopes)
	}

	sort.Sort(IntArr(envelopes))

	buf := make([]int, len(envelopes))
	max := 0
	buf[0] = 1
	for i := 1; i < len(envelopes); i++ {
		c := 0
		for j := 0; j < i; j++ {
			if envelopes[j][0] < envelopes[i][0] && envelopes[j][1] < envelopes[i][1] && buf[j] > c {
				c = buf[j]
			}
		}
		buf[i] = c + 1
		if c+1 > max {
			max = c + 1
		}
	}
	return max
}

func main() {
	fmt.Println(maxEnvelopes([][]int{[]int{5, 4}, []int{6, 4}, []int{6, 7}, []int{2, 3}}))
}
