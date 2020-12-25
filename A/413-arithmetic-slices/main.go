package main

import "fmt"

func numberOfArithmeticSlices(A []int) int {
	if len(A) <= 2 {
		return 0
	}
	s := 0
	sum := 0
	diff := A[1] - A[0]
	for i := 2; i < len(A); i++ {
		if A[i]-A[i-1] == diff {
			continue
		}
		e := i - 1
		n := e + 1 - s - 2
		sum += n * (n + 1) / 2
		s = i - 1
		diff = A[i] - A[i-1]
	}
	e := len(A) - 1
	n := e + 1 - s - 2
	sum += n * (n + 1) / 2
	return sum
}

func main() {
	fmt.Println()
}
