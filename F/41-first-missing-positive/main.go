package main

import "fmt"

func firstMissingPositive(nums []int) int {
	for i := 0; i < len(nums); i++ {
		s := nums[i]
		if s > len(nums) || s <= 0 {
			nums[i] = 0
		} else if i+1 != s {
			t := s
			nums[i] = 0
			for {
				b := nums[t-1]
				nums[t-1] = t
				if b > len(nums) || b <= 0 || nums[b-1] == b {
					break
				}
				t = b
			}
		}
	}

	for i := 0; i < len(nums); i++ {
		if nums[i] == 0 {
			return i + 1
		}
	}
	return len(nums) + 1
}

func main() {
	fmt.Println(firstMissingPositive([]int{1, 2, 0}))
	fmt.Println(firstMissingPositive([]int{3, 4, -1, 1}))
	fmt.Println(firstMissingPositive([]int{7, 8, 9, 10, 12}))
}
