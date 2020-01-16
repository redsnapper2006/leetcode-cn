package main

import (
	"fmt"
)

func productExceptSelf(nums []int) []int {
	size := len(nums)
	ret := make([]int, size)
	ret[0] = 1
	for i := 1; i < size; i++ {
		ret[i] = ret[i-1] * nums[i-1]
	}

	accum := 1
	for i := size - 2; i >= 0; i-- {
		accum *= nums[i+1]
		ret[i] = ret[i] * accum
	}
	return ret
}

func productExceptSelfV2(nums []int) []int {
	size := len(nums)
	leftArray := make([]int, size)
	rightArray := make([]int, size)
	accum := 1
	for i := 0; i < size-1; i++ {
		accum *= nums[i]
		leftArray[i] = accum
	}
	accum = 1
	for i := size - 1; i > 0; i-- {
		accum *= nums[i]
		rightArray[i] = accum
	}

	ret := make([]int, size)
	for i := 0; i < size; i++ {
		left := 1
		right := 1
		if i > 0 {
			left = leftArray[i-1]
		}
		if i < size-1 {
			right = rightArray[i+1]
		}
		ret[i] = left * right
	}
	return ret
}

func main() {
	fmt.Println(productExceptSelf([]int{1, 2, 3, 4, 5}))
}
