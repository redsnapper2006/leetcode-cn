package main

import "fmt"

func subarraySum(nums []int, k int) int {
	count := 0
	M := make(map[int]int)
	M[0] = 1
	sum := 0
	for i := 0; i < len(nums); i++ {
		sum += nums[i]
		if M[sum-k] > 0 {
			count += M[sum-k]
		}
		M[sum]++
	}

	return count
}

func subarraySumV2(nums []int, k int) int {
	buf := make([]int, len(nums))
	count := 0
	sum := 0
	for i := 0; i < len(nums); i++ {
		sum += nums[i]
		buf[i] = sum
		if sum == k {
			count++
		}
	}
	for i := 1; i < len(nums); i++ {
		for j := i; j < len(nums); j++ {
			if buf[j]-buf[i-1] == k {
				count++
			}
		}
	}
	return count
}

func main() {
	fmt.Println("a")
}
