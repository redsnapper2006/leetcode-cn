package main

import "fmt"

func isMonotonic(A []int) bool {
	if len(A) <= 1 {
		return true
	}

	direction := 0
	for i := 0; i < len(A)-1; i++ {
		if A[i] == A[i+1] {
			continue
		} else {
			if direction == 0 {
				if A[i] < A[i+1] {
					direction = 1
				} else {
					direction = -1
				}
			} else {
				if A[i] < A[i+1] && direction == -1 {
					return false
				}
				if A[i] > A[i+1] && direction == 1 {
					return false
				}
			}
		}
	}
	return true
}

func main() {
	fmt.Println("a")
}
