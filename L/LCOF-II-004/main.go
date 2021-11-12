package main

import "fmt"

func singleNumber(nums []int) int {
	ret := int32(0)
	for i := 0; i < 32; i++ {
		total := 0
		for j := 0; j < len(nums); j++ {
			if nums[j]&(1<<i) != 0 {
				total++
			}
		}
		if total%3 != 0 {
			ret |= 1 << i
		}
	}
	return int(ret)
}

func main() {
	fmt.Println()
}
