package main

import "fmt"

func minCostClimbingStairs(cost []int) int {
	buf := make([]int, len(cost))
	buf[0] = cost[0]
	buf[1] = cost[1]
	for i := 2; i < len(cost); i++ {
		d := buf[i-2]
		if d > buf[i-1] {
			d = buf[i-1]
		}
		buf[i] = d + cost[i]
	}
	ret := buf[len(cost)-1]
	if ret > buf[len(cost)-2] {
		ret = buf[len(cost)-2]
	}
	return ret
}

func main() {
	fmt.Println()
}
