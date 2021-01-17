package main

import (
	"fmt"
	"sort"
)

func minDeletions(s string) int {
	buf := make([]int, 26)
	for _, b := range s {
		buf[int(b-'a')]++
	}
	sort.Ints(buf)
	max := buf[len(buf)-1]
	sum := 0
	fmt.Println(buf)
	for i := len(buf) - 2; i >= 0; i-- {
		if buf[i] == 0 {
			break
		}
		if max <= buf[i] {
			if max > 0 {
				max--
			}
			sum += buf[i] - max
		} else {
			max = buf[i]
		}
	}
	return sum
}

func main() {
	fmt.Println()
}
