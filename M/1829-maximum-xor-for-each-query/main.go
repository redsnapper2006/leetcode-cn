package main

import "fmt"

func getMaximumXor(nums []int, maximumBit int) []int {
	MAX := 1<<maximumBit - 1
	ret := []int{}
	xorSum := nums[0]
	for i := 1; i < len(nums); i++ {
		xorSum ^= nums[i]
	}
	// fmt.Println("xor", xorSum)
	ret = append(ret, MAX^xorSum)
	// fmt.Println("0", ret)
	for i := len(nums) - 1; i > 0; i-- {
		xorSum ^= nums[i]
		// fmt.Println("xor loop", xorSum)
		ret = append(ret, MAX^xorSum)
	}
	return ret
}

func main() {
	fmt.Println(getMaximumXor([]int{0, 1, 1, 3}, 2))

	fmt.Println(getMaximumXor([]int{2, 3, 4, 7}, 3))
}
