package main

import (
	"fmt"
	"sort"
)

func maxSumRangeQuery(nums []int, requests [][]int) int {
	M := make([]int, len(nums))
	for _, c := range requests {
		M[c[0]]++
		if c[1] < len(nums)-1 {
			M[c[1]+1]--
		}
	}
	for i := 1; i < len(M); i++ {
		M[i] += M[i-1]
	}

	sort.Ints(M)
	sort.Ints(nums)
	sum := 0
	for i := len(M) - 1; i >= 0; i-- {
		sum += M[i] * nums[len(nums)-1-len(M)+1+i]
		sum %= 1000000007
	}
	return sum
}

func main() {
	fmt.Println()
}
