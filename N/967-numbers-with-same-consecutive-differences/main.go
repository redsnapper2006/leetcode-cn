package main

import (
	"fmt"
)

func numsSameConsecDiff(N int, K int) []int {
	if N == 1 {
		return []int{0, 1, 2, 3, 4, 5, 6, 7, 8, 9}
	}
	if K == 0 {
		var ret []int
		for i := 1; i < 10; i++ {
			b := i
			for j := 1; j < N; j++ {
				b = b*10 + i
			}
			ret = append(ret, b)
		}
		return ret
	}
	count := K
	if count > 10-K {
		count = 10 - K
	}
	candi := make([][]int, count)
	for i := 0; i < count; i++ {
		var t []int
		for j := i; j < 10; j = j + K {
			t = append(t, j)
		}
		candi = append(candi, t)
	}

	var recur func(N int, digits int, index int, nums []int) []int
	recur = func(N int, digits int, index int, nums []int) []int {
		if N > 1 && N == digits && nums[index] == 0 {
			return nil
		}
		if digits == 1 {
			return []int{nums[index]}
		}

		base := 1
		for i := 1; i < digits; i++ {
			base *= 10
		}
		var ret []int
		if index > 0 {
			left := recur(N, digits-1, index-1, nums)
			for i := 0; i < len(left); i++ {
				ret = append(ret, nums[index]*base+left[i])
			}
		}
		if index < len(nums)-1 {
			right := recur(N, digits-1, index+1, nums)
			for i := 0; i < len(right); i++ {
				ret = append(ret, nums[index]*base+right[i])
			}
		}
		return ret
	}
	var ret []int
	for i := 0; i < len(candi); i++ {
		for j := 0; j < len(candi[i]); j++ {
			r := recur(N, N, j, candi[i])
			ret = append(ret, r...)
		}
	}
	return ret
}

func main() {
	fmt.Println("a")
}
