package main

import "fmt"

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

func main() {
	fmt.Println("a")
}
