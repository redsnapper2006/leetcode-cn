package main

import "fmt"

func minDeletionSize(A []string) int {
	S := len(A[0])
	c := 0
	for i := 0; i < S; i++ {
		b := A[0][i]
		isValid := true
		for j := 1; j < len(A); j++ {
			if A[j][i] < b {
				isValid = false
				break
			}
			b = A[j][i]
		}
		if !isValid {
			c++
		}
	}
	return c
}

func main() {
	fmt.Println("a")
}
