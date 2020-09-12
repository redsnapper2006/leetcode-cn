package main

import "fmt"

func balancedStringSplit(s string) int {
	sum := 0
	var stack []byte
	for _, b := range s {
		if len(stack) > 0 && stack[len(stack)-1] != byte(b) {
			stack = stack[0 : len(stack)-1]
			if len(stack) == 0 {
				sum++
			}
		} else {
			stack = append(stack, byte(b))
		}
	}
	return sum
}

func main() {
	fmt.Println("a")
}
