package main

func minSubArrayLen(target int, nums []int) int {
	buf := make([]int, len(nums)+1)
	buf[0] = 0
	ret := len(nums) + 1

	for i := 0; i < len(nums); i++ {
		buf[i+1] = buf[i] + nums[i]
	}

	first, last := 0, 0
	for last <= len(nums) {
		if buf[last]-buf[first] < target {
			last++
		} else {
			if last-first < ret {
				ret = last - first
			}
			first++
		}
	}
	if ret == len(nums)+1 {
		return 0
	}
	return ret

}
