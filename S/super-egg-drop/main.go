package main

import (
	"fmt"
)

var (
	matrix [][]int
)

func superEggDrop(K int, N int) int {
	matrix = make([][]int, K+1)
	for i := 0; i < K+1; i++ {
		matrix[i] = make([]int, N+1)
		for j, _ := range matrix[i] {
			matrix[i][j] = -1
		}
	}
	return superEggDropRecur(K, N)
}

func superEggDropRecur(K int, N int) int {
	for i := 1; i <= N; i++ {
		matrix[1][i] = i
	}

	for i := 1; i <= K; i++ {
		matrix[i][1] = 1
		matrix[i][2] = 2
	}

	for i := 2; i <= K; i++ {
		for j := 3; j <= N; j++ {
			times := N + 1
			for m := 2; m <= j; m++ {
				left := matrix[i-1][m-1]
				right := matrix[i][j-m]
				times_t := -1
				if left > right {
					times_t = left + 1
				} else {
					times_t = right + 1
				}
				if times_t < times {
					times = times_t
				}
			}
			matrix[i][j] = times
		}
	}
	return matrix[K][N]
}

func main() {
	fmt.Println(superEggDrop(2, 2))

	fmt.Println(superEggDrop(100, 10000))
}
