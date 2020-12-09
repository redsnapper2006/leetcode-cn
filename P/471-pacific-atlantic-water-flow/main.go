package main

import "fmt"

func pacificAtlantic(matrix [][]int) [][]int {
	if len(matrix) == 0 {
		return nil
	}
	pac := make([][]int, len(matrix))
	atl := make([][]int, len(matrix))
	for i := 0; i < len(matrix); i++ {
		pac[i] = make([]int, len(matrix[0]))
		atl[i] = make([]int, len(matrix[0]))
	}

	for i := 0; i < len(matrix); i++ {
		pac[i][0] = 1
		atl[i][len(matrix[0])-1] = 1
	}
	for j := 0; j < len(matrix[0]); j++ {
		pac[0][j] = 1
		atl[len(matrix)-1][j] = 1
	}

	for {
		isPChg := false
		for i := 0; i < len(matrix); i++ {
			for j := 0; j < len(matrix[0]); j++ {
				if pac[i][j] == 1 {
					continue
				}
				if i > 0 && pac[i-1][j] == 1 && matrix[i][j] >= matrix[i-1][j] {
					pac[i][j] = 1
					isPChg = true
				}
				if i < len(matrix)-1 && pac[i+1][j] == 1 && matrix[i][j] >= matrix[i+1][j] {
					pac[i][j] = 1
					isPChg = true
				}
				if j > 0 && pac[i][j-1] == 1 && matrix[i][j] >= matrix[i][j-1] {
					pac[i][j] = 1
					isPChg = true
				}
				if j < len(matrix[0])-1 && pac[i][j+1] == 1 && matrix[i][j] >= matrix[i][j+1] {
					pac[i][j] = 1
					isPChg = true
				}
			}
		}
		if !isPChg {
			break
		}
	}

	for {
		isAChg := false
		for i := 0; i < len(matrix); i++ {
			for j := 0; j < len(matrix[0]); j++ {
				if atl[i][j] == 1 {
					continue
				}
				if i > 0 && atl[i-1][j] == 1 && matrix[i][j] >= matrix[i-1][j] {
					atl[i][j] = 1
					isAChg = true
				}
				if i < len(matrix)-1 && atl[i+1][j] == 1 && matrix[i][j] >= matrix[i+1][j] {
					atl[i][j] = 1
					isAChg = true
				}
				if j > 0 && atl[i][j-1] == 1 && matrix[i][j] >= matrix[i][j-1] {
					atl[i][j] = 1
					isAChg = true
				}
				if j < len(matrix[0])-1 && atl[i][j+1] == 1 && matrix[i][j] >= matrix[i][j+1] {
					atl[i][j] = 1
					isAChg = true
				}
			}
		}
		if !isAChg {
			break
		}
	}

	var ret [][]int
	for i := 0; i < len(matrix); i++ {
		for j := 0; j < len(matrix[0]); j++ {
			if pac[i][j] == 1 && atl[i][j] == 1 {
				ret = append(ret, []int{i, j})
			}
		}
	}
	return ret
}

func main() {
	fmt.Println("a")
}
