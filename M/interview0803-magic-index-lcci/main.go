package main

import "fmt"

func findMagicIndex(nums []int) int {
	for i := 0; i < len(nums); i++ {
		if nums[i] == i {
			return i
		}
	}
	return -1
}

func main() {
	fmt.Println("a")
}
