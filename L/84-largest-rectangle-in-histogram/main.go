package main

import "fmt"

func largestRectangleArea(heights []int) int {
	if len(heights) == 0 {
		return 0
	}

	var recur func(hs []int, s, e int) int
	recur = func(hs []int, s, e int) int {
		if e < s || s > e {
			return 0
		}
		// fmt.Println(s, e)
		min := hs[s]
		idx := s
		for i := s + 1; i <= e; i++ {
			if hs[i] < min {
				min = hs[i]
				idx = i
			}
		}
		v := min * (e - s + 1)
		vLeft := recur(hs, s, idx-1)
		vRight := recur(hs, idx+1, e)
		r := v
		if vLeft > r {
			r = vLeft
		}
		if vRight > r {
			r = vRight
		}
		return r
	}
	return recur(heights, 0, len(heights)-1)
}

func main() {
	fmt.Println(largestRectangleArea([]int{2, 1, 2}))
	fmt.Println(largestRectangleArea([]int{4, 2, 0, 3, 2, 4, 3, 4}))
	fmt.Println(largestRectangleArea([]int{0, 0, 0, 0, 0, 0, 0, 0, 2147483647}))
}
