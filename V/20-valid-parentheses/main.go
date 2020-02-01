package main

import (
	"fmt"
)

func isValid(s string) bool {
	if len(s) == 0 {
		return true
	}

	stack := []int{}
	for _, v := range s {
		if v == 40 || v == 91 || v == 123 {
			stack = append(stack, int(v))
		} else {
			if (v == 41 && (len(stack) < 1 || stack[len(stack)-1] != 40)) || (v == 93 && (len(stack) < 1 || stack[len(stack)-1] != 91)) || (v == 125 && (len(stack) < 1 || stack[len(stack)-1] != 123)) {
				return false
			} else {
				stack = stack[0 : len(stack)-1]
			}
		}
	}
	return len(stack) == 0
}

func main() {
	fmt.Println(isValid("]}}"))
}
