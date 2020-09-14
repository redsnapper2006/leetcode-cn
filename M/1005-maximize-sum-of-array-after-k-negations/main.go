package main

import (
	"fmt"
	"sort"
)

func largestSumAfterKNegations(A []int, K int) int {
	sort.Ints(A)
	steps := 0
	sum := 0
	for i := 0; steps < K && i < len(A); i++ {
		if A[i] < 0 {
			A[i] = -A[i]
			steps++
		}
	}
	K -= steps
	sort.Ints(A)
	if K%2 == 1 {
		A[0] = -A[0]
	}
	for _, c := range A {
		sum += c
	}

	return sum
}

func main() {
	fmt.Println("a")
}
