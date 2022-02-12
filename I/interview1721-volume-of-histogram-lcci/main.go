package main

import "fmt"

func trap(height []int) int {
	stack := [][]int{}
	cnt := 0

	s := 0
	for s < len(height)-1 {
		if height[s] <= height[s+1] {
			s++
		} else {
			break
		}
	}
	e := len(height) - 1
	for e > 0 {
		if height[e] <= height[e-1] {
			e--
		} else {
			break
		}
	}

	for i := s; i <= e; i++ {
		// fmt.Println("now", i)
		v := height[i]
		if len(stack) == 0 || stack[len(stack)-1][0] >= v {

		} else {
			// fmt.Println("clear stack", stack)
			prev := stack[len(stack)-1][0]
			for len(stack) > 0 && stack[len(stack)-1][0] <= v {
				p := stack[len(stack)-1]
				cnt += (p[0] - prev) * (i - p[1] - 1)
				prev = stack[len(stack)-1][0]
				stack = stack[0 : len(stack)-1]
			}
			if len(stack) > 0 {
				cnt += (v - prev) * (i - stack[len(stack)-1][1] - 1)
			}
			// fmt.Println("cnt", cnt)
		}
		stack = append(stack, []int{v, i})
	}
	return cnt
}

func main() {
	fmt.Println()
}
