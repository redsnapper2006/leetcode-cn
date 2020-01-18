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

func partition(s string) [][]string {
	var size int
	if isPalindrome(s) {
		size = 1
	} else {
		size = 0
	}
	c := make([][]string, size)

	for i := 1; i < len(s); i++ {

		if isPalindrome(s[0:i]) {
			right := partition(s[i:len(s)])
			newSize := size + len(right)
			t := make([][]string, newSize)
			copy(t, c)

			for n := 0; n < len(right); n++ {
				t[size+n] = append([]string{s[0:i]}, right[n]...)
			}
			size = newSize
			c = t
		}
	}

	if isPalindrome(s) {
		c[0] = []string{s}
	}

	return c
}

func main() {
	fmt.Println(partition("aab"))
}
