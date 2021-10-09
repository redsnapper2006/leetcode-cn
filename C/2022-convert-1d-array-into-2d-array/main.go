package main

import "fmt"

func construct2DArray(original []int, m int, n int) [][]int {
	if len(original) != m*n {
		return [][]int{}
	}
	ret := make([][]int, m)
	for i := 0; i < m; i++ {
		ret[i] = make([]int, n)
		copy(ret[i], original[i*m:i*m+n])
	}
	return ret
}

func main() {
	fmt.Println()
}
