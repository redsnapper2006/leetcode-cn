package main

import "fmt"

func sortArrayByParity(A []int) []int {
	s, e := 0, len(A)-1
	for s < e {
		for s < e && A[s]%2 == 0 {
			s++
		}
		for e > s && A[e]%2 == 1 {
			e--
		}
		if s < e {
			A[s], A[e] = A[e], A[s]
		}
	}
	return A
}

func main() {
	fmt.Println("a")
}
