package main

import (
	"fmt"
	"sort"
)

func threeSumClosest(nums []int, target int) int {

	if len(nums) <= 3 {
		sum := 0
		for i := 0; i < len(nums); i++ {
			sum += nums[i]
		}
		return sum
	}
	min := 1<<31 - 1
	sumMin := 0
	sort.Ints(nums)
	for i := 0; i < len(nums)-2; i++ {
		s, e := i+1, len(nums)-1
		for s < e {
			sum := nums[i] + nums[s] + nums[e]
			sub := nums[i] + nums[s] + nums[e] - target
			if sub == 0 {
				return target
			} else if sub > 0 {
				e--
			} else {
				s++
			}
			if sub < 0 {
				sub = -sub
			}
			if sub < min {
				min = sub
				sumMin = sum
			}
		}
	}
	return sumMin
}

func main() {
	fmt.Println(threeSumClosest([]int{-1, 2, 1, -4}, 1))
}
