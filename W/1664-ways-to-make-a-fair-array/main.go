package main

import "fmt"

func waysToMakeFair(nums []int) int {
	left, right := make([][]int, len(nums)), make([][]int, len(nums))

	even, odd := 0, 0
	for i, b := range nums {
		if i%2 == 0 {
			even += b
		} else {
			odd += b
		}
		left[i] = []int{even, odd}
	}
	even, odd = 0, 0
	for i := len(nums) - 1; i >= 0; i-- {
		if i%2 == 0 {
			even += nums[i]
		} else {
			odd += nums[i]
		}
		right[i] = []int{even, odd}
	}
	cnt := 0
	for i := 0; i < len(nums); i++ {
		peven, podd := 0, 0
		if i > 0 {
			peven, podd = left[i-1][0], left[i-1][1]
		}
		neven, nodd := 0, 0
		if i < len(nums)-1 {
			neven, nodd = right[i+1][0], right[i+1][1]
		}
		if peven+nodd == podd+neven {
			cnt++
		}
	}
	return cnt
}

func main() {
	fmt.Println()
}
