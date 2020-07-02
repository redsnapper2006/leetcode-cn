package main

import "fmt"

func missingNumber(nums []int) int {
	n := len(nums)

	sum := 0
	for _, v := range nums {
		sum += v
	}

	return n*(n+1)/2 - sum
}

func main() {
	fmt.Println("a")
}
