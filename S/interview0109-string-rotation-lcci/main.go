package main

import (
	"fmt"
	"strings"
)

func isFlipedString(s1 string, s2 string) bool {
	if s1 == s2 {
		return true
	}
	if len(s2) == 0 {
		return false
	}
	return strings.Index(s1+s1, s2) != -1
}

func main() {
	fmt.Println("a")
}
