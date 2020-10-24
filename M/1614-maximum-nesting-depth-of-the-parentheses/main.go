package main

import "fmt"

func maxDepth(s string) int {
	var stack []byte
	max := 0
	for _, b := range s {
		if b == '(' {
			stack = append(stack, byte(b))
			if len(stack) > max {
				max = len(stack)
			}
		} else if b == ')' {
			stack = stack[0 : len(stack)-1]
		}
	}
	return max
}

func main() {
	fmt.Println("a")
}
