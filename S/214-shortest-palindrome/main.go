package main

import (
	"fmt"
)

func shortestPalindrome(s string) string {
	if len(s) == 0 {
		return s
	}
	b := make([]byte, len(s))
	for i := 0; i < len(s); i++ {
		b[i] = s[len(s)-1-i]
	}
	for i := 0; i < len(s); i++ {
		isMatch := true
		for j := i; j < len(b); j++ {
			if b[j] != s[j-i] {
				isMatch = false
				break
			}
		}
		if isMatch {
			return string(b[0:i]) + s
		}
	}

	return ""
}

func main() {
	fmt.Println(shortestPalindrome("aacecaaa"))
	fmt.Println(shortestPalindrome("abcd"))
}
