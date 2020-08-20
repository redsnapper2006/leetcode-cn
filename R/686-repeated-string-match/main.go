package main

import (
	"fmt"
	"strings"
)

func repeatedStringMatch(A string, B string) int {
	s := A
	for len(s) < len(A)+len(B) {
		if strings.Index(s, B) != -1 {
			break
		}
		s += A
	}
	if strings.Index(s, B) == -1 {
		return -1
	}
	return len(s) / len(A)
}

func main() {
	fmt.Println("a")
}
