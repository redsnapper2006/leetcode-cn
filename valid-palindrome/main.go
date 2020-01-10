package main

import (
	"bytes"
	"fmt"
	"strings"
)

func isPalindrome(s string) bool {
	var b bytes.Buffer
	for _, v := range strings.ToLower(s) {
		if (v >= 97 && v <= 122) || (v >= 48 && v <= 57) {
			b.WriteRune(v)
		}
	}
	s_t := b.String()
	if len(s_t) == 0 {
		return true
	}
	mid := (len(s_t) + 1) / 2
	for i := 0; i < mid; i++ {
		if s_t[i] != s_t[len(s_t)-1-i] {
			return false
		}
	}
	return true
}

func main() {
	fmt.Println("hello")
	fmt.Println(isPalindrome("023349P"))
}
