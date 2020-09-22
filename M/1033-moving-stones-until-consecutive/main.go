package main

import (
	"fmt"
	"sort"
)

func numMovesStones(a int, b int, c int) []int {
	buf := []int{a, b, c}
	sort.Ints(buf)
	max := buf[2] - buf[1] - 1 + buf[1] - buf[0] - 1
	x, y, z := buf[0], buf[1], buf[2]
	min := 0

	left := y - x
	right := z - y
	if left == 1 && right == 1 {
	} else if left == 1 || right == 1 || right == 2 || left == 2 {
		min = 1
	} else {
		min = 2
	}

	return []int{min, max}
}

func main() {
	fmt.Println("a")
}
