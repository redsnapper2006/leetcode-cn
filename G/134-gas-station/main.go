package main

import "fmt"

func canCompleteCircuit(gas []int, cost []int) int {
	buf := make([]int, len(gas))
	sum := 0
	for i := 0; i < len(gas); i++ {
		buf[i] = gas[i] - cost[i]
		sum += buf[i]
	}
	if sum < 0 {
		return -1
	}
	first := -1
	for i := 0; i < len(buf); i++ {
		if buf[i] >= 0 {
			first = i
			break
		}
	}

	as := 0
	index := -1
	for i := first; i < first+len(buf); i++ {
		idx := i % len(buf)
		if buf[idx] >= 0 && index == -1 {
			as = 0
			index = idx
		}
		as += buf[idx]
		if as < 0 {
			index = -1
		}
	}
	return index
}

func main() {
	fmt.Println("a")
}
