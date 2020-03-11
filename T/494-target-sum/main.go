package main

import "fmt"

func findTargetSumWays(nums []int, S int) int {

	var recur func(nums []int, S int) int
	recur = func(nums []int, S int) int {
		if len(nums) == 1 {
			t := 0
			if nums[0] == S {
				t++
			}
			if nums[0] == -S {
				t++
			}
			return t
		}
		c := nums[0]
		remain := nums[1:]
		return recur(remain, S+c) + recur(remain, S-c)
	}
	return recur(nums, S)
}

func main() {
	fmt.Println("a")
}
