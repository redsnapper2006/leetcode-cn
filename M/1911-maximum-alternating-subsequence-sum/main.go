package main

import "fmt"

func maxAlternatingSum(nums []int) int64 {
	s := 0
	var sum int64 = 0

	for s < len(nums) {
		for s < len(nums)-1 {
			if nums[s] <= nums[s+1] {
				s++
			} else {
				break
			}
		}
		sum += int64(nums[s])
		s++

		for s < len(nums)-1 {
			if nums[s] >= nums[s+1] {
				s++
			} else {
				break
			}
		}
		if s < len(nums)-1 {
			sum -= int64(nums[s])
		}
		s++
	}
	return sum
}

func maxAlternatingSumBetter(nums []int) int64 {
	var even int64 = int64(nums[0])
	var odd int64 = 0

	for i := 1; i < len(nums); i++ {
		newodd := odd
		if even-int64(nums[i]) > odd {
			newodd = even - int64(nums[i])
		}
		neweven := even
		if odd+int64(nums[i]) > even {
			neweven = odd + int64(nums[i])
		}
		odd, even = newodd, neweven
	}
	return even
}

func main() {
	fmt.Println()
}
