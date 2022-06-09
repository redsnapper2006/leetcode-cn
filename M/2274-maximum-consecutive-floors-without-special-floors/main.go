package main

import "sort"

func maxConsecutive(bottom int, top int, special []int) int {
	// buf := make([]int, len(special)+2)
	buf := append(special, bottom-1, top+1)
	sort.Ints(buf)
	max := 0
	for i := 1; i < len(buf); i++ {
		if buf[i]-buf[i-1] < 2 {
			continue
		}
		if buf[i]-buf[i-1]-1 > max {
			max = buf[i] - buf[i-1] - 1
		}
	}
	return max
}
