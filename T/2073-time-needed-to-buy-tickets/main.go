package main

import (
	"fmt"
	"sort"
)

func timeRequiredToBuy(tickets []int, k int) int {
	prev := 0
	base := tickets[k]
	for i := 0; i < k; i++ {
		if tickets[i] >= tickets[k] {
			prev++
		}
	}
	sort.Ints(tickets)

	idx := 0
	seconds := 0
	for i := 0; i < len(tickets); i++ {
		if tickets[i] < base {
			seconds += tickets[i]
			continue
		}
		idx = i
		break
	}

	seconds += (len(tickets)-idx)*(base-1) + prev + 1
	return seconds
}

func main() {
	fmt.Println()
}
