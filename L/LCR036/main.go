package main

import (
	"fmt"
	"strconv"
)

func evalRPN(tokens []string) int {
	stack := []int{}
	for _, s := range tokens {
		if s == "+" || s == "-" || s == "*" || s == "/" {
			a, b := stack[len(stack)-2], stack[len(stack)-1]
			r := 0
			if s == "+" {
				r = a + b
			}
			if s == "-" {
				r = a - b
			}
			if s == "*" {
				r = a * b
			}
			if s == "/" {
				r = a / b
			}
			stack = stack[0 : len(stack)-2]
			stack = append(stack, r)
		} else {
			v, _ := strconv.ParseInt(s, 10, 64)
			val := int(v)
			stack = append(stack, val)
		}
	}
	return stack[0]
}

func main() {
	fmt.Println()
}
