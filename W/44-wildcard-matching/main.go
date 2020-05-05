package main

import (
	"fmt"
)

func isMatch(s string, p string) bool {
	buf := make([][]int, len(p)+1)
	for i := 0; i < len(p)+1; i++ {
		buf[i] = make([]int, len(s)+1)
	}
	buf[0][0] = 1
	for i := 0; i < len(p); i++ {
		if p[i] == '*' && buf[i][0] == 1 {
			buf[i+1][0] = 1
		}
	}
	for i := 1; i < len(p)+1; i++ {
		for j := 1; j < len(s)+1; j++ {
			if p[i-1] == '?' {
				buf[i][j] = buf[i-1][j-1]
			} else if p[i-1] == '*' && (buf[i-1][j-1] == 1 || buf[i-1][j] == 1 || buf[i][j-1] == 1) {
				buf[i][j] = 1
			} else if p[i-1] == s[j-1] {
				buf[i][j] = buf[i-1][j-1]
			}
		}
	}
	// fmt.Println("buf", buf)
	return buf[len(p)][len(s)] == 1
}

func main() {
	fmt.Println(isMatch("aa", "a"))
	fmt.Println(isMatch("aa", "*"))
	fmt.Println(isMatch("cb", "?a"))
	fmt.Println(isMatch("adceb", "*a*b"))
	fmt.Println(isMatch("acdcb", "a*c?b"))
}
