package main

import "fmt"

func transpose(A [][]int) [][]int {
	buf := make([][]int, len(A[0]))
	for i := 0; i < len(buf); i++ {
		buf[i] = make([]int, len(A))
	}
	for i := 0; i < len(A); i++ {
		for j := 0; j < len(A[0]); j++ {
			buf[j][i] = A[i][j]
		}
	}
	return buf
}

func main() {
	fmt.Println("a")
}
