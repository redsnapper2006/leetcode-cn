package main

import "fmt"

func smallestRangeI(A []int, K int) int {
	min, max := 1<<31-1, -1
	for i := 0; i < len(A); i++ {
		if min > A[i] {
			min = A[i]
		}
		if max < A[i] {
			max = A[i]
		}
	}
	if max-min-2*K <= 0 {
		return 0
	}

	return max - min - 2*K
}

func main() {
	fmt.Println("a")
}
