package main

import (
	"fmt"
	"sort"
)

type RangeSlice [][]int

func (a RangeSlice) Len() int {
	return len(a)
}
func (a RangeSlice) Swap(i, j int) {
	a[i], a[j] = a[j], a[i]
}
func (a RangeSlice) Less(i, j int) bool {
	return a[i][0] < a[j][0]
}
func longestValidParentheses(s string) int {
	if len(s) == 0 {
		return 0
	}

	var stack []int
	var buf [][]int
	for i, v := range s {
		if v == 40 {
			// (
			stack = append(stack, i)
		} else {
			// )
			if len(stack) > 0 {
				buf = append(buf, []int{stack[len(stack)-1], i})
				stack = stack[0 : len(stack)-1]
			}
		}
	}
	if len(buf) == 0 {
		return 0
	}

	sort.Sort(RangeSlice(buf))

	start := buf[0][0]
	end := buf[0][1]
	max := end - start
	for _, v := range buf {
		if v[0] >= start && v[1] <= end {
			continue
		}

		if v[0] == end+1 {
			end = v[1]
		} else {
			if max < end-start {
				max = end - start
			}
			start = v[0]
			end = v[1]
		}
	}
	if max < end-start {
		max = end - start
	}
	return max + 1
}

func main() {
	// 40 41
	// fmt.Println(longestValidParentheses(")(()())((()))(()))"))
	fmt.Println(longestValidParentheses("))"))
}
