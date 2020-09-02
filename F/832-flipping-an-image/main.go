package main

import "fmt"

func flipAndInvertImage(A [][]int) [][]int {
	buf := make([][]int, len(A))

	for i := 0; i < len(A); i++ {
		buf[i] = make([]int, len(A[0]))
		for j := 0; j < len(A[0]); j++ {
			buf[i][j] = 1 - A[i][len(A)-1-j]
		}
	}
	return buf
}

func main() {
	fmt.Println("a")
}
