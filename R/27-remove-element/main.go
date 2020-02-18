package main

import (
	"fmt"
)

func removeElement(nums []int, val int) int {
	s := 0
	for i := 0; i < len(nums); i++ {
		if nums[i] != val {
			nums[s] = nums[i]
			s++
		}
	}
	return s
}

func main() {
	fmt.Println(removeElement([]int{3, 2, 2, 3}, 3))
}
