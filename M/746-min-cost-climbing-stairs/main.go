package main

import (
	"fmt"
)

func minCostClimbingStairs(cost []int) int {
	size := len(cost)

	buf := make([]int, size)
	buf[0] = cost[0]
	buf[1] = cost[1]
	for i := 2; i < size; i++ {
		if buf[i-2] > buf[i-1] {
			buf[i] = buf[i-1] + cost[i]
		} else {
			buf[i] = buf[i-2] + cost[i]
		}
	}
	if buf[size-1] > buf[size-2] {
		return buf[size-2]
	} else {
		return buf[size-1]
	}
}

func minCostClimbingStairsV2(cost []int) int {
	size := len(cost)
	if size < 2 {
		return 0
	}

	first := cost[size-1] + minCostClimbingStairs(cost[0:size-1])

	second := cost[size-2] + minCostClimbingStairs(cost[0:size-2])

	if first > second {
		return second
	} else {
		return first
	}
}

func main() {
	fmt.Println(minCostClimbingStairs([]int{10, 15, 20}))
	fmt.Println(minCostClimbingStairs([]int{1, 100, 1, 1, 1, 100, 1, 1, 100, 1}))
}
