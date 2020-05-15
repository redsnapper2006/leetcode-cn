package main

import (
	"fmt"
	"sort"
)

func hIndex(citations []int) int {
	if len(citations) == 0 {
		return 0
	}
	if len(citations) == 1 {
		if citations[0] >= 1 {
			return 1
		}
		return 0
	}
	sort.Ints(citations)
	for i := len(citations) - 1; i >= 1; i-- {
		c := len(citations) - i
		if citations[i] >= c && citations[i-1] <= c {
			return c
		}
	}
	if citations[0] >= len(citations) {
		return len(citations)
	}
	return 0
}

func main() {
	fmt.Println("a")
}
