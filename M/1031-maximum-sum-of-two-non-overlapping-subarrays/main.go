package main

import "fmt"

func maxSumTwoNoOverlap(nums []int, firstLen int, secondLen int) int {
	buf := make([]int, len(nums))
	sum := 0
	for i := 0; i < len(nums); i++ {
		sum += nums[i]
		buf[i] = sum
	}
	left1 := make([]int, len(nums))
	max := 0
	for i := firstLen - 1; i < len(nums); i++ {
		v := 0
		if i >= firstLen {
			v = buf[i] - buf[i-firstLen]
		} else {
			v = buf[i]
		}
		if v > max {
			max = v
		}
		left1[i] = max
	}
	right1 := make([]int, len(nums))
	max = 0
	for i := len(nums) - secondLen; i >= 0; i-- {
		v := 0
		if i > 0 {
			v = buf[i+secondLen-1] - buf[i-1]
		} else {
			v = buf[i+secondLen-1]
		}
		if v > max {
			max = v
		}
		right1[i] = max
	}
	
	ret := 0
	for i := 0; i < len(nums)-1; i++ {
		if ret < left1[i]+right1[i+1] {
			ret = left1[i] + right1[i+1]
		}
	}

	left2 := make([]int, len(nums))
	max = 0
	for i := secondLen - 1; i < len(nums); i++ {
		v := 0
		if i >= secondLen {
			v = buf[i] - buf[i-secondLen]
		} else {
			v = buf[i]
		}
		if v > max {
			max = v
		}
		left2[i] = max
	}
	right2 := make([]int, len(nums))
	max = 0
	for i := len(nums) - firstLen; i >= 0; i-- {
		v := 0
		if i > 0 {
			v = buf[i+firstLen-1] - buf[i-1]
		} else {
			v = buf[i+firstLen-1]
		}
		if v > max {
			max = v
		}
		right2[i] = max
	}
	for i := 0; i < len(nums)-1; i++ {
		if ret < left2[i]+right2[i+1] {
			ret = left2[i] + right2[i+1]
		}
	}
	return ret
}

func main() {
	fmt.Println()
}
