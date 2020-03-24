package main

import "fmt"

func minimumTotal(triangle [][]int) int {
	if len(triangle) == 0 {
		return 0
	}

	if len(triangle) == 1 {
		return triangle[0][0]
	}

	BUF := make([]int, len(triangle))
	BUF[0] = triangle[0][0]
	for i := 1; i < len(triangle); i++ {
		var prev int
		for j := 0; j < len(triangle[i]); j++ {
			left, right := 1<<31-1, 1<<31-1
			if j > 0 {
				left = triangle[i][j] + prev
			}
			if j < len(triangle[i])-1 {
				right = triangle[i][j] + BUF[j]
			}
			prev = BUF[j]
			if left > right {
				BUF[j] = right
			} else {
				BUF[j] = left
			}
		}
	}

	MIN := 1<<31 - 1
	for i := 0; i < len(BUF); i++ {
		if BUF[i] < MIN {
			MIN = BUF[i]
		}
	}
	return MIN
}

func main() {
	fmt.Println(minimumTotal([][]int{[]int{2}, []int{3, 4}, []int{6, 5, 7}, []int{4, 1, 8, 3}}))
}
