package main

import (
	"fmt"
)

func dailyTemperatures(T []int) []int {
	if len(T) == 0 {
		return []int{0}
	}
	r := make([]int, len(T))
	var stack []int
	stack = append(stack, 0)
	for i := 1; i < len(T); i++ {
		for len(stack) > 0 && T[i] > T[stack[len(stack)-1]] {
			r[stack[len(stack)-1]] = i - stack[len(stack)-1]
			stack = stack[0 : len(stack)-1]
		}

		stack = append(stack, i)
	}
	for len(stack) > 0 {
		r[stack[len(stack)-1]] = 0
		stack = stack[0 : len(stack)-1]
	}
	return r
}

func main() {
	fmt.Println(dailyTemperatures([]int{73, 74, 75, 71, 69, 72, 76, 73}))
}
