package main

import "fmt"

func maximumGap(nums []int) int {
	if len(nums) < 2 {
		return 0
	}
	min, max := 1<<31-1, -1
	for _, v := range nums {
		if min > v {
			min = v
		}
		if max < v {
			max = v
		}
	}
	size := (max - min) / (len(nums) - 1)
	if size == 0 {
		size = 1
	}
	count := (max-min)/size + 1

	buf := make([][]int, count)
	for i := 0; i < len(buf); i++ {
		buf[i] = make([]int, 2)
	}

	for _, v := range nums {
		idx := (v - min) / size
		if buf[idx][0] == 0 {
			buf[idx][0] = v
		} else if buf[idx][0] > v {
			buf[idx][0] = v
		}
		if buf[idx][1] == 0 {
			buf[idx][1] = v
		} else if buf[idx][1] < v {
			buf[idx][1] = v
		}
	}
	ret := 0
	for _, v := range buf {
		min, max := v[0], v[1]
		if min == 0 || max == 0 {
			continue
		}
		if ret < max-min {
			ret = max - min
		}
	}
	prev := -1
	for i := 0; i < len(buf); i++ {
		if buf[i][0] == 0 {
			continue
		}
		if prev != -1 {
			if buf[i][0]-buf[prev][1] > ret {
				ret = buf[i][0] - buf[prev][1]
			}
		}
		prev = i
	}
	return ret
}

func main() {
	fmt.Println()
}
