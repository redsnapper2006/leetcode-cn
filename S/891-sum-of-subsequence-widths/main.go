package main

import (
	"fmt"
	"sort"
)

func sumSubseqWidths(nums []int) int {
	M := 1000000007
	sort.Ints(nums)
	n := len(nums)
	sum := 0
	pow2 := make([]int, n)
	pow2[0] = 1
	for i := 1; i < n; i++ {
		pow2[i] = pow2[i-1] * 2 % M
	}

	for i := 0; i < n; i++ {
		fmt.Println(pow2[i], pow2[n-1-i], nums[i], ((pow2[i] - pow2[n-1-i]) * nums[i]))
		sum += ((pow2[i] - pow2[n-1-i]) * nums[i])
	}
	return (sum%M + M) % M
}

func main() {
	fmt.Println(sumSubseqWidths([]int{5, 69, 89, 92, 31, 16, 25, 45, 63,
		40, 16, 56, 24, 40, 75, 82, 40, 12, 50, 62, 92, 44, 67, 38, 92, 22,
		91, 24, 26, 21, 100, 42, 23, 56, 64, 43, 95}))
}
