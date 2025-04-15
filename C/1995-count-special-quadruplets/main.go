package main

import "fmt"

func countQuadruplets(nums []int) int {
	ret := 0
	for i, a := range nums {
		for j := i + 1; j < len(nums); j++ {
			for m := j + 1; m < len(nums); m++ {
				for n := m + 1; n < len(nums); n++ {
					if a+nums[j]+nums[m] == nums[n] {
						ret++
					}
				}
			}
		}
	}
	return ret
}

func main() {
	fmt.Println()
}
