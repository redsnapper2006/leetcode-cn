package main

import "fmt"

func diagonalSum(mat [][]int) int {
	size := len(mat)
	sum := 0
	for i := 0; i < size; i++ {
		sum += mat[i][i]
		sum += mat[size-1-i][i]
	}
	if size%2 == 1 {
		sum -= mat[size/2][size/2]
	}
	return sum
}

func main() {
	fmt.Println("a")
}
