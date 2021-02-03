package main

import (
	"fmt"
	"sort"
)

func medianSlidingWindow(nums []int, k int) []float64 {
	buf := make([]int, k)
	for i := 0; i < k; i++ {
		buf[i] = nums[i]
	}
	sort.Ints(buf)

	var ret []float64
	if k%2 == 0 {
		ret = append(ret, float64(buf[k/2-1]+buf[k/2])/2)
	} else {
		ret = append(ret, float64(buf[k/2]))
	}

	for i := k; i < len(nums); i++ {
		remove := nums[i-k]
		s, e := 0, k-1
		idx := -1
		for s <= e {
			m := s + (e-s)/2
			if buf[m] < remove {
				s = m + 1
			} else if buf[m] > remove {
				e = m - 1
			} else {
				idx = m
				break
			}
		}

		buf = append(buf[0:idx], buf[idx+1:]...)
		s, e = 0, len(buf)-1
		for s <= e {
			m := s + (e-s)/2
			if buf[m] < nums[i] {
				s = m + 1
			} else if buf[m] >= nums[i] {
				e = m - 1
			}
		}
		t := make([]int, k)
		copy(t, buf[0:s])
		t[s] = nums[i]
		copy(t[s+1:], buf[s:])
		buf = t
		if k%2 == 0 {
			ret = append(ret, float64(buf[k/2-1]+buf[k/2])/2)
		} else {
			ret = append(ret, float64(buf[k/2]))
		}
	}
	return ret
}

func main() {
	fmt.Println()
}
