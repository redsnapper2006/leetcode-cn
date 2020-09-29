package main

import "fmt"

func createTargetArray(nums []int, index []int) []int {
	buf := make([]int, len(nums))
	buf[0] = nums[0]
	for i := 1; i < len(index); i++ {
		copy(buf[index[i]+1:], buf[index[i]:i+1])
		buf[index[i]] = nums[i]
	}
	return buf
}

func main() {
	fmt.Println("a")
}
