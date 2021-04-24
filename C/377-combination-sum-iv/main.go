package main

import "fmt"

func combinationSum4(nums []int, target int) int {
	buf := make([]int, target+1)
	buf[0] = 1
	for i := 0; i <= target; i++ {
		if buf[i] == 0 {
			continue
		}
		for j := 0; j < len(nums); j++ {
			if i+nums[j] > target {
				continue
			}
			buf[i+nums[j]] += buf[i]
		}
	}
	return buf[target]
}

func main() {
	fmt.Println(combinationSum4([]int{1, 2, 3}, 4))
}
