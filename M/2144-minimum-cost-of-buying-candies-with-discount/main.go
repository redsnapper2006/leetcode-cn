package main

import "sort"

func minimumCost(cost []int) int {
	sort.Ints(cost)

	sum := 0
	for i := len(cost) - 1; i >= 0; i-- {
		if (len(cost)-1-i)%3 != 2 {
			sum += cost[i]
		}
	}
	return sum
}

func minimumCost2(cost []int) int {
	sort.Ints(cost)
	buf := make([]int, len(cost))

	sum := 0
	for i := len(cost) - 1; i >= 0; i-- {
		if buf[i] == 1 {
			continue
		}
		sum += cost[i]
		buf[i] = 1
		idx := -1
		for j := i - 1; j >= 0; j-- {
			if buf[j] == 0 {
				idx = j
				break
			}
		}
		if idx > -1 {
			sum += cost[idx]
			buf[idx] = 1
			gidx := -1
			for j := idx - 1; j >= 0; j-- {
				if buf[j] == 0 {
					gidx = j
					break
				}
			}
			if gidx > -1 {
				buf[gidx] = 1
			}
		}
	}
	return sum
}
