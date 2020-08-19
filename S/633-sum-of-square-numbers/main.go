package main

import (
	"fmt"
	"math"
)

func judgeSquareSum(c int) bool {
	for i := 0; i*i <= c; i++ {
		remain := c - i*i
		r := int(math.Sqrt(float64(remain)))
		if r*r == remain {
			return true
		}
	}
	return false
}

func main() {
	fmt.Println("a")
}
