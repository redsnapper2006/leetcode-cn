package main

import "fmt"

func nextGreaterElements(nums []int) []int {
	if len(nums) == 0 {
		return nil
	}
	if len(nums) == 1 {
		return []int{-1}
	}
	ret := make([]int, len(nums))
	stack := []int{nums[0]}
	pos := []int{0}

	for i := 1; i < len(nums); i++ {
		if nums[i] > stack[len(stack)-1] {
			idx := len(stack) - 1
			for idx >= 0 && stack[idx] < nums[i] {
				ret[pos[idx]] = nums[i]
				idx--
			}
			stack = stack[0 : idx+1]
			pos = pos[0 : idx+1]
		}
		stack = append(stack, nums[i])
		pos = append(pos, i)
	}

	if len(stack) > 0 {
		for i := 0; i <= pos[0]; i++ {
			if nums[i] > stack[len(stack)-1] {
				idx := len(stack) - 1
				for idx >= 0 && stack[idx] < nums[i] {
					ret[pos[idx]] = nums[i]
					idx--
				}
				stack = stack[0 : idx+1]
				pos = pos[0 : idx+1]
			}
		}
		for i := 0; i < len(pos); i++ {
			ret[pos[i]] = -1
		}
	}
	return ret
}

func main() {
	fmt.Println("a")
}
