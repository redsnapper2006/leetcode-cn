package main

import "fmt"

func numSubarrayProductLessThanK(nums []int, k int) int {
	if k <= 1 {
		return 0
	}
	first, last := 0, 0
	ret := 0
	mul := 1
	for last < len(nums) {
		mul *= nums[last]
		for mul >= k {
			mul /= nums[first]
			first++
		}
		ret += last - first + 1
		last++
	}
	return ret
}

func numSubarrayProductLessThanK2(nums []int, k int) int {
	first, last := 0, 0
	ret := 0
	mul := 1
	for first < len(nums) {
		if first > last {
			mul = 1
			last = first
		}
		for last < len(nums) {
			mul *= nums[last]
			if mul < k {
				last++
			} else {
				mul /= nums[last]
				break
			}
		}
		ret += last - first
		mul /= nums[first]
		first++
	}
	return ret
}

func main() {
	fmt.Println(numSubarrayProductLessThanK([]int{10, 5, 2, 6}, 100))
}
