package main

import (
	"fmt"
)

func kthSmallest(matrix [][]int, k int) int {
	size := len(matrix)

	if k == 1 {
		return matrix[0][0]
	}
	if k == size*size {
		return matrix[size-1][size-1]
	}

	l, r := matrix[0][0], matrix[size-1][size-1]
	for l < r {
		// fmt.Println(l, r)
		accum := 0
		mid := (l + r) / 2

		i := size - 1
		j := 0
		for i >= 0 && j < size {
			if matrix[i][j] > mid {
				accum += j
				i--
			} else {
				j++
				if j == size {
					accum += size * (i + 1)
				}
			}
		}
		if accum >= k {
			r = mid
		} else {
			l = mid + 1
		}
	}
	return l
}

func kthSmallestV2(matrix [][]int, k int) int {
	size := len(matrix)
	if k == 1 {
		return matrix[0][0]
	}
	if k == size*size {
		return matrix[size-1][size-1]
	}

	rowIndex := make([]int, size)
	accum := 0
	var ret int
	rowCandi := make([]int, size)
	for i := 0; i < size; i++ {
		rowCandi[i] = i
	}

	for accum < k {
		min := matrix[rowCandi[0]][rowIndex[rowCandi[0]]]
		// fmt.Println("min", min)
		moveIndex := rowCandi[0]
		index := 0
		for i := 1; i < len(rowCandi); i++ {
			if min > matrix[rowCandi[i]][rowIndex[rowCandi[i]]] {
				min = matrix[rowCandi[i]][rowIndex[rowCandi[i]]]
				moveIndex = rowCandi[i]
				index = i
			}
		}

		ret = matrix[moveIndex][rowIndex[moveIndex]]

		accum++
		rowIndex[moveIndex]++
		if rowIndex[moveIndex] >= size {
			// rowIndex = append(rowIndex[0:moveIndex], rowIndex[moveIndex+1:]...)
			rowCandi = append(rowCandi[0:index], rowCandi[index+1:]...)
		}
	}
	return ret
}

func main() {

	// fmt.Println(kthSmallest([][]int{
	// 	{1, 5, 9},
	// 	{10, 11, 14},
	// 	{12, 13, 15},
	// }, 9))
	fmt.Println(kthSmallest([][]int{
		{1, 5, 9},
		{10, 11, 14},
		{12, 13, 15},
	}, 8))
	fmt.Println(kthSmallest([][]int{
		{1, 5, 9},
		{10, 11, 14},
		{12, 13, 15},
	}, 7))
	fmt.Println(kthSmallest([][]int{
		{1, 5, 9},
		{10, 11, 14},
		{12, 13, 15},
	}, 6))
	fmt.Println(kthSmallest([][]int{
		{1, 5, 9},
		{10, 11, 14},
		{12, 13, 15},
	}, 5))
	fmt.Println(kthSmallest([][]int{
		{1, 5, 9},
		{10, 11, 14},
		{12, 13, 15},
	}, 4))
	fmt.Println(kthSmallest([][]int{
		{1, 5, 9},
		{10, 11, 14},
		{12, 13, 15},
	}, 3))
	fmt.Println(kthSmallest([][]int{
		{1, 5, 9},
		{10, 11, 14},
		{12, 13, 15},
	}, 2))
	fmt.Println(kthSmallest([][]int{
		{1, 4, 7, 11, 15},
		{2, 5, 8, 12, 19},
		{3, 6, 9, 16, 22},
		{10, 13, 14, 17, 24},
		{18, 21, 23, 26, 30},
	}, 20))

	fmt.Println(kthSmallest([][]int{
		{1, 2, 3, 4, 5},
		{6, 7, 8, 9, 10},
		{11, 12, 13, 14, 15},
		{16, 17, 18, 19, 20},
		{21, 22, 23, 24, 25},
	}, 20))
}
