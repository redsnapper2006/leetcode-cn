package main

import (
	"fmt"
)

func rotate(nums []int, k int) {
	length := len(nums)
	o := k % length
	b := make([]int, o)
	copy(b, nums[length-o:length])
	copy(nums[o:length], nums[0:length-o])
	copy(nums[0:o], b)
	fmt.Println(nums)
}

func main() {
	rotate([]int{1, 2, 3, 4, 5, 6, 7}, 3)
}
