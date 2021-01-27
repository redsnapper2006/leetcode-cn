package main

import (
	"fmt"
	"sort"
)

func kthLargestValue(matrix [][]int, k int) int {
	buf := make([][]int, len(matrix))
	for i := 0; i < len(matrix); i++ {
		buf[i] = make([]int, len(matrix[0]))
	}

	var ret []int
	for i := 0; i < len(matrix); i++ {
		line := 0
		for j := 0; j < len(matrix[0]); j++ {
			line ^= matrix[i][j]
			p := 0
			if i > 0 {
				p = buf[i-1][j]
			}
			buf[i][j] = p ^ line
			ret = append(ret, buf[i][j])
		}
	}
	sort.Ints(ret)
	return ret[len(ret)-k]
}

func main() {
	fmt.Println()
}
