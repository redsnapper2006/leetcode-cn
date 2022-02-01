package main

import "fmt"

func maxCoins(nums []int) int {
	n := len(nums)
	cache := make([][]int, n+2)
	for i := 0; i < n+2; i++ {
		cache[i] = make([]int, n+2)
	}

	val := make([]int, n+2)
	val[0], val[n+1] = 1, 1
	for i := 1; i < n+1; i++ {
		val[i] = nums[i-1]
	}

	for i := n - 1; i >= 0; i-- {
		for j := i + 2; j < n+2; j++ {
			for k := i + 1; k < j; k++ {
				sum := val[i] * val[k] * val[j]
				sum += cache[i][k] + cache[k][j]
				if sum > cache[i][j] {
					cache[i][j] = sum
				}
			}
		}
	}
	return cache[0][n+1]
}

func main() {
	fmt.Println()
}
