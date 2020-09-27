package main

import "fmt"

func validMountainArray(A []int) bool {
	if len(A) < 3 {
		return false
	}
	s, e := 1, len(A)-2
	for i := 1; i < len(A); i++ {
		if A[i] <= A[i-1] {
			s = i - 1
			break
		}
	}
	for i := len(A) - 2; i >= 0; i-- {
		if A[i] <= A[i+1] {
			e = i + 1
			break
		}
	}

	return s == e
}

func main() {
	fmt.Println("a")
}
