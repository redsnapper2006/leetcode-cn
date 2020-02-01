package main

import (
	"fmt"
)

func permute(nums []int) [][]int {
	if len(nums) == 1 {
		return [][]int{nums}
	}
	var ret [][]int
	for i := 0; i < len(nums); i++ {
		buf := make([]int, len(nums)-1)
		copy(buf, nums[0:i])
		copy(buf[i:], nums[i+1:])
		r := permute(buf)
		for _, v := range r {
			ret = append(ret, append(v, nums[i]))
		}
	}
	return ret
}

func main() {
	fmt.Println(permute([]int{1, 2, 3}))
}
