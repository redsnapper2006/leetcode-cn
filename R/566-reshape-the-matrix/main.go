package main

import "fmt"

func matrixReshape(nums [][]int, r int, c int) [][]int {
	row := len(nums)
	col := len(nums[0])
	if row*col != r*c {
		return nums
	}

	buf := make([][]int, r)

	for i := 0; i < r; i++ {
		buf[i] = make([]int, c)
		for j := 0; j < c; j++ {
			o := i*c + j
			oc := o % col
			or := o / col
			buf[i][j] = nums[or][oc]
		}
	}
	return buf
}

func main() {
	fmt.Println("a")
}
