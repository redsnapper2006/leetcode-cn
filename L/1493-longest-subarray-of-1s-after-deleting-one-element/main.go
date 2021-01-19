package main

import "fmt"

func longestSubarrayV2(nums []int) int {
	idx := -1
	for i, b := range nums {
		if b == 0 {
			continue
		}
		idx = i
		break
	}
	if idx == -1 {
		return 0
	}
	var buf [][]int
	s := idx
	for i := idx; i < len(nums); i++ {
		if nums[i] == 1 {
			if s == -1 {
				s = i
			}
			continue
		}
		if s != -1 {
			buf = append(buf, []int{s, i - 1})
		}
		s = -1
	}
	if s != -1 {
		buf = append(buf, []int{s, len(nums) - 1})
	}
	if len(buf) == 1 {
		if buf[0][1] == len(nums)-1 && buf[0][0] == 0 {
			return buf[0][1] - buf[0][0]
		}
		return buf[0][1] - buf[0][0] + 1
	}

	max := -1
	for i := 1; i < len(buf); i++ {
		b, p := buf[i], buf[i-1]
		if b[0]-p[1] == 2 {
			if max < b[1]-p[0] {
				max = b[1] - p[0]
			}
		} else {
			if max < p[1]-p[0]+1 {
				max = p[1] - p[0] + 1
			}
		}
	}
	b := buf[len(buf)-1]
	if max < b[1]-b[0]+1 {
		max = b[1] - b[0] + 1
	}
	return max
}

func longestSubarray(nums []int) int {
	p1, p2 := 0, 0
	max := 0
	for _, b := range nums {
		if b == 0 {
			p2 = p1
			p1 = 0
		} else {
			p1++
			p2++
			if max < p2 {
				max = p2
			}
		}
	}
	if max == len(nums) {
		max--
	}
	return max
}

func main() {
	fmt.Println()
}
