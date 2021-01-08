package main

import (
	"fmt"
)

func rotateV2(nums []int, k int) {
	length := len(nums)
	o := k % length
	b := make([]int, o)
	copy(b, nums[length-o:length])
	copy(nums[o:length], nums[0:length-o])
	copy(nums[0:o], b)
	fmt.Println(nums)
}

func rotate(nums []int, k int) {
	k %= len(nums)
	s, e := 0, len(nums)-1
	for s < e {
		nums[s], nums[e] = nums[e], nums[s]
		s++
		e--
	}

	s, e = 0, k-1
	for s < e {
		nums[s], nums[e] = nums[e], nums[s]
		s++
		e--
	}

	s, e = k, len(nums)-1
	for s < e {
		nums[s], nums[e] = nums[e], nums[s]
		s++
		e--
	}
}

func main() {
	rotate([]int{1, 2, 3, 4, 5, 6, 7}, 3)
}
