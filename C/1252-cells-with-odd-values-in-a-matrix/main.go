package main

import (
	"fmt"
)

func oddCells(n int, m int, indices [][]int) int {
	rMap, cMap := make(map[int]int), make(map[int]int)
	for i := 0; i < len(indices); i++ {
		rMap[indices[i][0]]++
		cMap[indices[i][1]]++
	}

	rOdd, rEven, cOdd, cEven := 0, 0, 0, 0
	for i := 0; i < n; i++ {
		if rMap[i]%2 == 0 {
			rEven++
		} else {
			rOdd++
		}
	}
	for i := 0; i < m; i++ {
		if cMap[i]%2 == 0 {
			cEven++
		} else {
			cOdd++
		}
	}

	return rOdd*cEven + rEven*cOdd
}

func main() {
	fmt.Println(oddCells(2, 3, [][]int{{0, 1}, {1, 1}}))
	fmt.Println(oddCells(2, 2, [][]int{{0, 0}, {1, 1}}))
}
