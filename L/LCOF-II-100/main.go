package main

import (
	"fmt"
)

func minimumTotal(triangle [][]int) int {
	buf := make([]int, len(triangle))
	buf[0] = triangle[0][0]
	for i := 1; i < len(triangle); i++ {
		buf[i] = buf[i-1]
		for j := len(triangle[i]) - 2; j >= 1; j-- {
			min := triangle[i][j] + buf[j]
			if min > triangle[i][j]+buf[j-1] {
				min = triangle[i][j] + buf[j-1]
			}
			buf[j] = min
		}
		buf[0] = buf[0] + triangle[i][0]
		buf[i] = buf[i] + triangle[i][len(triangle[i])-1]
	}

	min := 1<<31 - 1
	for j := 0; j < len(buf); j++ {
		if buf[j] < min {
			min = buf[j]
		}
	}
	return min
}

func main() {
	fmt.Println()
}
