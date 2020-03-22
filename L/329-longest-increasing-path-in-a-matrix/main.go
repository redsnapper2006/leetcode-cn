package main

import (
	"fmt"
	"sort"
)

type SortBy [][]int

func (a SortBy) Len() int           { return len(a) }
func (a SortBy) Swap(i, j int)      { a[i], a[j] = a[j], a[i] }
func (a SortBy) Less(i, j int) bool { return a[i][0] < a[j][0] }

func longestIncreasingPath(matrix [][]int) int {
	buf := make([][]int, len(matrix))
	for i := 0; i < len(matrix); i++ {
		buf[i] = make([]int, len(matrix[0]))
	}

	var sortRC [][]int
	for i := 0; i < len(matrix); i++ {
		for j := 0; j < len(matrix[0]); j++ {
			sortRC = append(sortRC, []int{matrix[i][j], i, j})
		}
	}
	sort.Sort(SortBy(sortRC))

	retMax := 0
	for i := 0; i < len(sortRC); i++ {
		r, c := sortRC[i][1], sortRC[i][2]
		max := 0
		if r > 0 && matrix[r-1][c] < matrix[r][c] && buf[r-1][c] > max {
			max = buf[r-1][c]
		}
		if c > 0 && matrix[r][c-1] < matrix[r][c] && buf[r][c-1] > max {
			max = buf[r][c-1]
		}
		if r < len(matrix)-1 && matrix[r+1][c] < matrix[r][c] && buf[r+1][c] > max {
			max = buf[r+1][c]
		}
		if c < len(matrix[0])-1 && matrix[r][c+1] < matrix[r][c] && buf[r][c+1] > max {
			max = buf[r][c+1]
		}
		buf[r][c] = max + 1
		if buf[r][c] > retMax {
			retMax = buf[r][c]
		}
	}

	return retMax
}

func main() {
	fmt.Println(longestIncreasingPath([][]int{
		{3, 4, 5},
		{3, 2, 6},
		{2, 2, 1},
	}))
}
