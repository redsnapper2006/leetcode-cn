package main

import (
	"fmt"
)

func moveZeroes(nums []int) {
	zeroIndex := 0
	for i, v := range nums {
		if v != 0 {
			if i > zeroIndex {
				nums[zeroIndex] = v
			}
			zeroIndex++
		}
	}
	for i := zeroIndex; i < len(nums); i++ {
		nums[i] = 0
	}
	fmt.Println(nums)
}

func main() {
	moveZeroes([]int{13, -1, 0, 3, 12, 0})
}
