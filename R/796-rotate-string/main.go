package main

import (
	"fmt"
	"strings"
)

func rotateString(A string, B string) bool {
	return len(A) == len(B) && (strings.Index(A+A, B) > 0 && strings.Index(A+A, B) < len(A) || A == B)
}

func main() {
	fmt.Println("a")
}
