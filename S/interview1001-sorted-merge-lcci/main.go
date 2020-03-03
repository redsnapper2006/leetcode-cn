package main

import "fmt"

func merge(A []int, m int, B []int, n int) {
	idx := len(A) - 1
	sA, sB := m-1, n-1

	for sA >= 0 && sB >= 0 {
		if A[sA] > B[sB] {
			A[idx] = A[sA]
			sA--
		} else {
			A[idx] = B[sB]
			sB--
		}
		idx--
	}
	if sA >= 0 {
		for i := sA; i >= 0; i-- {
			A[idx] = A[i]
			idx--
		}
	}
	if sB >= 0 {
		for i := sB; i >= 0; i-- {
			A[idx] = B[i]
			idx--
		}
	}
}

func main() {
	fmt.Println("a")
}
