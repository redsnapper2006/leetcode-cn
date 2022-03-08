package main

import "fmt"

func platesBetweenCandles(s string, queries [][]int) []int {
	buf := make([]int, len(s))

	sum := 0
	left, right := make([]int, len(s)), make([]int, len(s))
	lIdx, rIdx := -1, len(s)
	for i := 0; i < len(s); i++ {
		if s[i] == byte('*') {
			sum++
		}
		if s[i] == byte('|') && i > lIdx {
			lIdx = i
		}
		if s[len(s)-1-i] == byte('|') && (len(s)-1-i) < rIdx {
			rIdx = len(s) - 1 - i
		}
		buf[i] = sum

		left[i] = lIdx
		right[len(s)-1-i] = rIdx
	}

	ret := []int{}
	for _, q := range queries {
		l, r := q[0], q[1]
		if right[l] >= left[r] {
			ret = append(ret, 0)
		} else {
			ret = append(ret, buf[left[r]]-buf[right[l]])
		}
	}
	return ret
}

func main() {
	fmt.Println()
}
