package main

import (
	"fmt"
)

func getKthMagicNumber(k int) int {
	buf := make([]int, k)
	buf[0] = 1
	i3, i5, i7 := 0, 0, 0
	for i := 1; i < k; i++ {
		v3, v5, v7 := buf[i3]*3, buf[i5]*5, buf[i7]*7
		min := v3
		if min > v5 {
			min = v5
		}
		if min > v7 {
			min = v7
		}
		buf[i] = min
		if min == v3 {
			i3++
		}
		if min == v5 {
			i5++
		}
		if min == v7 {
			i7++
		}
	}
	return buf[k-1]
}

func main() {
	fmt.Println(getKthMagicNumber(3))
}
