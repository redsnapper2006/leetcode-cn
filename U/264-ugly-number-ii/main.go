package main

import (
	"fmt"
)

func nthUglyNumber(n int) int {
	buf := make([]int, n)
	buf[0] = 1
	i2, i3, i5 := 0, 0, 0
	for i := 1; i < n; i++ {
		v2, v3, v5 := buf[i2]*2, buf[i3]*3, buf[i5]*5
		min := v2
		if min > v3 {
			min = v3
		}
		if min > v5 {
			min = v5
		}
		buf[i] = min
		if min == v2 {
			i2++
		}
		if min == v3 {
			i3++
		}
		if min == v5 {
			i5++
		}
	}
	return buf[n-1]
}

func main() {
	fmt.Println(nthUglyNumber(1352))
}
