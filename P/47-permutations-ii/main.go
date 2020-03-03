package main

import (
	"fmt"
	"sort"
)

func permute(nums []int) [][]int {
	if len(nums) == 1 {
		return [][]int{nums}
	}
	var ret [][]int
	for i := 0; i < len(nums); i++ {
		if i > 0 && nums[i] == nums[i-1] {
			continue
		}
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

func permuteUnique(nums []int) [][]int {
	if len(nums) == 1 {
		return [][]int{nums}
	}
	sort.Ints(nums)
	return permute(nums)
}

func main() {
	fmt.Println(permuteUnique([]int{1, 1, 2, 2}))
}
