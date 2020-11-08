package main

import "fmt"

func findLUSlength(a string, b string) int {
	if a == b {
		return -1
	}
	if len(a) == len(b) {
		return len(a)
	}
	if len(a) > len(b) {
		return len(a)
	}
	return len(b)
}

func main() {
	fmt.Println()
}
