package main

import (
	"fmt"
	"sort"
)

func maxmiumScore(cards []int, cnt int) int {
	sort.Ints(cards)
	odd := []int{}
	even := []int{}
	for _, v := range cards {
		if v%2 == 0 {
			even = append(even, v)
		} else {
			odd = append(odd, v)
		}
	}
	for i := len(even) - 2; i >= 0; i-- {
		even[i] = even[i] + even[i+1]
	}
	for i := len(odd) - 2; i >= 0; i-- {
		odd[i] = odd[i] + odd[i+1]
	}

	start := cnt
	if start > len(even) {
		start = len(even)
	}
	max := 0
	for i := len(even) - start; i < len(even); i++ {
		remain := cnt - len(even) + i
		if remain%2 == 1 || remain > len(odd) {
			continue
		}

		v := even[i]
		if remain > 0 {
			v += odd[len(odd)-remain]
		}
		if max < v {
			max = v
		}
	}

	if cnt%2 == 0 && cnt <= len(odd) && max < odd[len(odd)-cnt] {
		max = odd[len(odd)-cnt]
	}

	return max
}

func main() {
	fmt.Println()
}
