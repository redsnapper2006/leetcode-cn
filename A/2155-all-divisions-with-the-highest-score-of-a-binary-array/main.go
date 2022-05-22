package main

func maxScoreIndices(nums []int) []int {
	zeros, ones := make([]int, len(nums)), make([]int, len(nums))

	cnt := 0
	for i := 0; i < len(nums); i++ {
		if nums[i] == 0 {
			cnt++
		}
		zeros[i] = cnt
	}

	cnt = 0
	for i := len(nums) - 1; i >= 0; i-- {
		if nums[i] == 1 {
			cnt++
		}
		ones[i] = cnt
	}

	ret := []int{}

	max := -1
	for i := 0; i < len(nums)+1; i++ {
		left := 0
		if i > 0 {
			left = zeros[i-1]
		}
		right := 0
		if i < len(nums) {
			right = ones[i]
		}
		if left+right > max {
			max = left + right
			ret = []int{i}
		} else if max == left+right {
			ret = append(ret, i)
		}
	}
	return ret
}
