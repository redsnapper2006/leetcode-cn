package main

import (
	"fmt"
)

func maxRotateFunction(A []int) int {
	sum := 0
	accum := 0
	for i := 0; i < len(A); i++ {
		sum += i * A[i]
		accum += A[i]
	}
	max := sum
	for i := 1; i < len(A); i++ {
		sum -= accum
		sum += len(A) * A[i-1]
		if sum > max {
			max = sum
		}
	}
	return max
}

func main() {
	fmt.Println("a")
}
