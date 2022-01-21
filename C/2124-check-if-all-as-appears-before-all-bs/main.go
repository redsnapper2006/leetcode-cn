package main

import "fmt"

func checkString(s string) bool {
	a, b := -1, len(s)
	for i := 0; i < len(s); i++ {
		if s[i] == byte('b') {
			b = i
			break
		}
	}
	for i := len(s) - 1; i >= 0; i-- {
		if s[i] == byte('a') {
			a = i
			break
		}
	}
	return a < b
}

func main() {
	fmt.Println()
}
