package main

import (
	"fmt"
)

func removeKdigits(num string, k int) string {
	if len(num) == 0 || k >= len(num) {
		return "0"
	}

	stack := []byte{num[0]}
	idx := 0
	for i := 1; i < len(num) && k > 0; i++ {
		if num[i] >= stack[len(stack)-1] {
			stack = append(stack, num[i])
		} else {
			s := len(stack) - 1
			for k > 0 && s >= 0 && stack[s] > num[i] {
				s--
				k--
			}
			stack[s+1] = num[i]
			stack = stack[0 : s+1+1]
		}
		idx++
	}
	for i := idx + 1; i < len(num); i++ {
		stack = append(stack, num[i])
	}

	stack = stack[0 : len(stack)-k]

	s := -1
	for i := 0; i < len(stack); i++ {
		if stack[i] != '0' {
			s = i
			break
		}
	}
	if s == -1 {
		return "0"
	}
	return string(stack[s:])
}

func main() {
	fmt.Println("a")
}
