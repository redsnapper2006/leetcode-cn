package main

import "fmt"

func repeatedNTimes(A []int) int {
	M := map[int]int{}
	for i := 0; i < len(A); i++ {
		_, ok := M[A[i]]
		if !ok {
			M[A[i]]++
		} else {
			return A[i]
		}
	}

	return 0
}

func main() {
	fmt.Println("a")
}
