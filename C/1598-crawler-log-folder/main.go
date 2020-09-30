package main

import "fmt"

func minOperations(logs []string) int {
	var stack []string

	for _, v := range logs {
		if v == "../" {
			if len(stack) > 0 {
				stack = stack[0 : len(stack)-1]
			}
		} else if v != "./" {
			stack = append(stack, v)
		}
	}
	return len(stack)
}

func main() {
	fmt.Println("a")
}
