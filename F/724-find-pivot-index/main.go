package main

import (
	"fmt"
)

func pivotIndex(nums []int) int {
	if len(nums) == 0 {
		return -1
	}

	buf := make([]int, len(nums)+2)
	buf[0] = 0
	buf[len(nums)+1] = 0
	accum := 0
	for i, v := range nums {
		accum += v
		buf[i+1] = accum
	}

	for i := 0; i < len(nums); i++ {
		if buf[i] == buf[len(buf)-2]-buf[i+1] {
			return i
		}
	}
	return -1
}

func main() {
	fmt.Println("a")
}
