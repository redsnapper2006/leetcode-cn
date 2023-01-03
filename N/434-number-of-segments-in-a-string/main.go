package main

import "fmt"

func countSegments(s string) int {
	c := 0
	sIdx := 0
	for sIdx < len(s) {
		if s[sIdx] != ' ' {
			c++
			for sIdx < len(s) && s[sIdx] != ' ' {
				sIdx++
			}
		}
		sIdx++
	}
	return c
}

func main() {
	fmt.Println("a")
}
