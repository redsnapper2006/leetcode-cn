package main

func subArrayRanges(nums []int) int64 {
	leftMinBuf := make([]int, len(nums))
	stack := []int{}
	for i, v := range nums {
		for len(stack) > 0 {
			if nums[stack[len(stack)-1]] > v {
				stack = stack[0 : len(stack)-1]
			} else {
				break
			}
		}
		if len(stack) == 0 {
			leftMinBuf[i] = -1
		} else {
			leftMinBuf[i] = stack[len(stack)-1]
		}
		stack = append(stack, i)
	}

	rightMinBuf := make([]int, len(nums))
	stack = []int{}
	for i := len(nums) - 1; i >= 0; i-- {
		for len(stack) > 0 {
			if nums[stack[len(stack)-1]] >= nums[i] {
				stack = stack[0 : len(stack)-1]
			} else {
				break
			}
		}
		if len(stack) == 0 {
			rightMinBuf[i] = len(nums)
		} else {
			rightMinBuf[i] = stack[len(stack)-1]
		}
		stack = append(stack, i)
	}

	leftMaxBuf := make([]int, len(nums))
	stack = []int{}
	for i, v := range nums {
		for len(stack) > 0 {
			if nums[stack[len(stack)-1]] <= v {
				stack = stack[0 : len(stack)-1]
			} else {
				break
			}
		}
		if len(stack) == 0 {
			leftMaxBuf[i] = -1
		} else {
			leftMaxBuf[i] = stack[len(stack)-1]
		}
		stack = append(stack, i)
	}

	rightMaxBuf := make([]int, len(nums))
	stack = []int{}
	for i := len(nums) - 1; i >= 0; i-- {
		for len(stack) > 0 {
			if nums[stack[len(stack)-1]] < nums[i] {
				stack = stack[0 : len(stack)-1]
			} else {
				break
			}
		}
		if len(stack) == 0 {
			rightMaxBuf[i] = len(nums)
		} else {
			rightMaxBuf[i] = stack[len(stack)-1]
		}
		stack = append(stack, i)
	}

	var sumMax, sumMin int64 = 0, 0
	for i := 0; i < len(nums); i++ {
		sumMax += int64(i-leftMaxBuf[i]) * int64(rightMaxBuf[i]-i) * int64(nums[i])
		sumMin += int64(i-leftMinBuf[i]) * int64(rightMinBuf[i]-i) * int64(nums[i])
	}

	return sumMax - sumMin
}

func subArrayRanges2(nums []int) int64 {
	var ret int64 = 0
	for i := 0; i < len(nums); i++ {
		min, max := nums[i], nums[i]
		for j := i; j >= 0; j-- {
			if min > nums[j] {
				min = nums[j]
			}
			if max < nums[j] {
				max = nums[j]
			}
			ret += int64(max - min)
		}
	}
	return ret
}
