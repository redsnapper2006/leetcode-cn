package main

import (
	"fmt"
	"sort"
)

func heightChecker(heights []int) int {
	buf := make([]int, len(heights))
	copy(buf, heights)
	sort.Ints(buf)
	steps := 0
	for i := 0; i < len(buf); i++ {
		if buf[i] != heights[i] {
			steps++
		}
	}
	return steps
}

func main() {
	fmt.Println("a")
}
