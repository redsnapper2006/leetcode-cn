package main

import (
	"fmt"
	"sort"
)

func maxProduct(nums []int) int {
	sort.Ints(nums)
	A, B := (nums[0]-1)*(nums[1]-1), (nums[len(nums)-2]-1)*(nums[len(nums)-1]-1)
	r := A
	if r < B {
		r = B
	}

	return r
}

func main() {
	fmt.Println("a")
}
