package main

import (
	"fmt"
	"sort"
)

func maxProfit(inventory []int, orders int) int {
	M := map[int]int{}
	for _, v := range inventory {
		M[v]++
	}
	var buf []int
	for k := range M {
		buf = append(buf, k)
	}
	sort.Ints(buf)
	count := 0
	sum := 0
	idx := len(buf) - 1
	for count < orders && idx >= 0 {
		start, end := 0, 0
		cnt := 0
		if idx == 0 {
			end = buf[0]
			cnt = M[buf[0]]
		} else {
			end = buf[idx]
			start = buf[idx-1]
			cnt = M[buf[idx]]
		}
		if count+(end-start)*cnt <= orders {
			sum += (start + 1 + end) * (end - start) / 2 * cnt
			count += (end - start) * cnt
			sum %= 1000000007
		} else {
			news := (orders - count) / cnt
			offset := (orders - count) % cnt
			sum += (end - news + 1 + end) * news / 2 * cnt
			// fmt.Println("sum1", sum)
			count += news * cnt
			sum %= 1000000007
			sum += (end - news) * offset
			sum %= 1000000007
			count += offset
		}
		idx--
		M[start] += cnt
		// buf = buf[0 : len(buf)-1]
	}
	return sum
}

func main() {
	fmt.Println()
}
