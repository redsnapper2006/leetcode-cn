package main

import "fmt"

func reverseParentheses(s string) string {
	pstack := []int{}
	buf := []byte{}

	for i := 0; i < len(s); i++ {
		if s[i] == byte('(') {
			pstack = append(pstack, len(buf))
		} else if s[i] == byte(')') {
			start := pstack[len(pstack)-1]
			pstack = pstack[0 : len(pstack)-1]
			end := len(buf) - 1
			for start < end {
				buf[start], buf[end] = buf[end], buf[start]
				start++
				end--
			}
		} else {
			buf = append(buf, s[i])
		}
	}
	return string(buf)
}

func main() {
	fmt.Println()
}
