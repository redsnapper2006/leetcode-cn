package main

import "fmt"

import "strings"

func gcdOfStrings(str1 string, str2 string) string {
	b1, b2 := str1, str2
	if len(str1) < len(str2) {
		b1, b2 = str2, str1
	}
	if !strings.HasPrefix(b1, b2) {
		return ""
	}
	for strings.HasPrefix(b1, b2) {
		b1 = b1[len(b2):]
	}
	if b1 == "" {
		return b2
	}
	return gcdOfStrings(b2, b1)
}

func main() {
	fmt.Println("a")
}
