package main

import "fmt"

func countNicePairs(nums []int) int {
	rev := make([]int, len(nums))
	for i := 0; i < len(nums); i++ {
		n := nums[i]
		b := 0
		for n > 0 {
			m := n % 10
			b = b*10 + m
			n /= 10
		}
		rev[i] = b
	}
	// fmt.Println(nums, rev)
	ret := 0
	m := map[int]int{}
	for i := 0; i < len(nums); i++ {
		m[nums[i]-rev[i]]++
	}
	// fmt.Println(m)
	for _, v := range m {
		ret += (v * (v - 1) / 2)
		ret %= 1000000007
	}

	return ret
}

func main() {
	fmt.Println(countNicePairs([]int{13, 10, 35, 24, 76}))
}
