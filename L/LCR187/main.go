package main

import (
	"fmt"
)

func lastRemaining(n int, m int) int {
	idx := 0

	for i := 1; i < n; i++ {
		idx = (m + idx) % (i + 1)
	}

	return idx
}

func main() {
	fmt.Println(lastRemaining(10, 17))
}
