package main

import "fmt"

func smallestEqual(nums []int) int {
	for i, v := range nums {
		if i%10 == v {
			return i
		}
	}
	return -1
}

func main() {
	fmt.Println()
}
