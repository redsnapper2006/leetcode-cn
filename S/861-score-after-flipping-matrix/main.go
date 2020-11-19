package main

import "fmt"

func matrixScore(A [][]int) int {
	for i := 0; i < len(A); i++ {
		if A[i][0] == 0 {
			for j := 0; j < len(A[0]); j++ {
				A[i][j] = 1 - A[i][j]
			}
		}
	}
	for i := 1; i < len(A[0]); i++ {
		sum := 0
		for j := 0; j < len(A); j++ {
			sum += A[j][i]
		}
		if sum <= len(A)/2 {
			for m := 0; m < len(A); m++ {
				for n := i; n < len(A[0]); n++ {
					A[m][n] = 1 - A[m][n]
				}
			}
		}
	}
	sum := 0
	for i := 0; i < len(A); i++ {
		s := 0
		for j := 0; j < len(A[0]); j++ {
			s = s*2 + A[i][j]
		}
		sum += s
	}
	return sum
}

func main() {
	fmt.Println()
}
