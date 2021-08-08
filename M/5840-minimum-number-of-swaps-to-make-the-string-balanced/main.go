package main

import "fmt"

func minSwaps(s string) int {
	buf := []byte(s)
	stack := []byte{}
	for _, v := range buf {
		if v == byte('[') {
			stack = append(stack, v)
		} else {
			if len(stack) > 0 && stack[len(stack)-1] == byte('[') {
				stack = stack[0 : len(stack)-1]
			} else {
				stack = append(stack, v)
			}
		}
	}
	if len(stack) == 0 {
		return 0
	}
	return (len(stack)/2 + 1) / 2
}

func main() {
	fmt.Println()
}
