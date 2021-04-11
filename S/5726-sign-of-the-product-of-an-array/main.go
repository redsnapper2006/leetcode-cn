package main

import "fmt"

func arraySign(nums []int) int {
	plus := 1
	for i := 0; i < len(nums); i++ {
		if nums[i] == 0 {
			return 0
		}
		if nums[i] < 0 {
			plus = -plus
		}
	}
	return plus
}

func main() {
	fmt.Println(arraySign([]int{1, 2, 3}))
}
