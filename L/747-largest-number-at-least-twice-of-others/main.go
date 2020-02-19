package main

import (
	"fmt"
)

func dominantIndex(nums []int) int {
	if len(nums) == 0 {
		return -1
	}
  if len(nums) == 1 {
		return 0
	}
	idx := 0
	max1, max2 := nums[0], nums[1]
	if max1 < max2 {
		max1, max2 = nums[1], nums[0]
		idx = 1
	}
  for i := 2; i < len(nums); i++ {
		if nums[i] > max1 {
			max2 = max1
			max1 = nums[i]
			idx = i
		} else if nums[i] > max2 {
			max2 = nums[i]
		}
	}

	if max1 >= max2*2 {
		return idx
	} else {
		return -1
	}
}

func main() {
	fmt.Println("a")
}
