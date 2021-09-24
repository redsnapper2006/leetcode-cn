package main

import "fmt"

func removeOccurrences(s string, part string) string {
	stack := []byte{}

	for i := 0; i < len(s); i++ {
		stack = append(stack, byte(s[i]))
		if byte(s[i]) == part[len(part)-1] && len(stack) >= len(part) {
			isMatch := true
			for j := len(part) - 1; j >= 0; j-- {
				if byte(part[j]) != stack[len(stack)-len(part)+j] {
					isMatch = false
					break
				}
			}
			if isMatch {
				stack = stack[0 : len(stack)-len(part)]
			}
		}
	}
	return string(stack)
}

func main() {
	fmt.Println()
}
