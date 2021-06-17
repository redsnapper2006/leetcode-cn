package main

import (
	"fmt"
)



func minAddToMakeValid(S string) int {
	var stack []byte

	for i := 0; i < len(S); i++ {
		if S[i] == '(' {
			stack = append(stack, S[i])
		} else {
			if len(stack) > 0 && stack[len(stack)-1] == '(' {
				stack = stack[0 : len(stack)-1]
			} else {
				stack = append(stack, S[i])
			}
		}
	}
	return len(stack)
}

func main() {
	fmt.Println("a")
}
