package main

import "fmt"

func scoreOfParentheses(S string) int {
	var stack []int
	for _, b := range S {
		if b == '(' {
			stack = append(stack, 0)
		} else {
			c := stack[len(stack)-1]
			if c == 0 {
				stack[len(stack)-1] = 1
			} else {
				stack[len(stack)-2] = c * 2
				stack = stack[0 : len(stack)-1]
			}
			idx := len(stack) - 1
			sum := 0
			for idx >= 0 && stack[idx] > 0 {
				sum += stack[idx]
				idx--
			}
			stack[idx+1] = sum
			stack = stack[0 : idx+2]
		}
	}
	return stack[0]
}

func main() {
	fmt.Println()
}
