package main

import (
	"fmt"
)

func rangeBitwiseAnd(m int, n int) int {
	ret := 0

	for i := 30; i >= 0; i-- {
		b := 1 << i
		if (m & b) == (n & b) {
			ret += (m & b)
		} else {
			break
		}
	}
	return ret
}

func main() {
	fmt.Println("a")
}
