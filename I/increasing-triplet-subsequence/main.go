package main

import (
	"fmt"
)

func increasingTriplet(nums []int) bool {
	if (len(nums) < 3) {
		return false
	}

	c1 := 0
	var c2 int
	for i:= 1; i< len(nums); i++ {
		if nums[i] < nums[ c1] {
			c1 =i
		}
		if c2 == 0 && nums[i] > nums[ c1]  {
			c2 = i
		}
		if c2 > 0 && nums[i] > nums[c2] {
			return true
		}
		if c2 > 0 && nums[i] < nums[c2] {
			if nums[i] > nums[c1] {
				c2 = i
			}
		}
	}
	return false
}

func main() {
	fmt.Println(increasingTriplet([]int{5,4,3,2,1}))
}
