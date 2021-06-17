package main

import (
	"fmt"
)

func findRotation(mat [][]int, target [][]int) bool {
	isMatch := true
	for i := 0; i < len(mat); i++ {
		for j := 0; j < len(mat[0]); j++ {
			if mat[i][j] != target[i][j] {
				isMatch = false
				break
			}
		}
		if !isMatch {
			break
		}
	}
	if isMatch {
		return true
	}
	isMatch = true
	for i := 0; i < len(mat); i++ {
		for j := 0; j < len(mat[0]); j++ {
			if mat[i][j] != target[j][len(mat[0])-1-i] {
				isMatch = false
				break
			}
		}
		if !isMatch {
			break
		}
	}
	if isMatch {
		return true
	}
	isMatch = true
	for i := 0; i < len(mat); i++ {
		for j := 0; j < len(mat[0]); j++ {
			if mat[i][j] != target[len(mat)-1-i][len(mat[0])-1-j] {
				isMatch = false
				break
			}
		}
		if !isMatch {
			break
		}
	}
	if isMatch {
		return true
	}
	isMatch = true
	for i := 0; i < len(mat); i++ {
		for j := 0; j < len(mat[0]); j++ {
			if mat[i][j] != target[len(mat)-1-j][i] {
				isMatch = false
				break
			}
		}
		if !isMatch {
			break
		}
	}
	return isMatch
}

func main() {
	fmt.Println()
}
