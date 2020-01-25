package main

import (
	"fmt"
	"sort"
)

func maxSlidingWindow(nums []int, k int) []int {
	if len(nums) == 0 || k == 0 {
		return nil
	}

	var ret, windows []int
	for i, v := range nums {
		if len(windows) > 0 && i-k == windows[0] {
			windows = windows[1:]
		}
		for len(windows) > 0 && nums[windows[len(windows)-1]] <= v {
			windows = windows[0 : len(windows)-1]
		}
		windows = append(windows, i)
		if i >= k-1 {
			ret = append(ret, nums[windows[0]])
		}
	}
	return ret
}

func maxSlidingWindowV2(nums []int, k int) []int {
	if len(nums) == 0 || k == 0 {
		return []int{}
	}
	ret := make([]int, len(nums)+1-k)
	for i := k - 1; i < len(nums); i++ {
		buf := make([]int, k)
		copy(buf, nums[i-k+1:i+1])
		sort.Ints(buf)
		ret[i-k+1] = buf[k-1]
	}
	return ret
}

func main() {
	fmt.Println(maxSlidingWindow([]int{1, 3, -1, -3, 5, 3, 6, 7}, 3))
}
