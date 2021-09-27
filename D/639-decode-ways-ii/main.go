package main

import (
	"fmt"
)

func numDecodings(s string) int {
	M := 1000000007
	buf := make([]int, len(s)+2)
	buf[len(s)] = 1
	buf[len(s)+1] = 0
	for i := len(s) - 1; i >= 0; i-- {
		if s[i] == '0' {
			buf[i] = 0
		} else if s[i] == '*' {
			one := (9 * buf[i+1]) % M
			two := 0
			if i < len(s)-1 {
				if s[i+1] >= '0' && s[i+1] <= '6' {
					two = (2 * buf[i+2]) % M
				} else if s[i+1] >= '7' && s[i+1] <= '9' {
					two = buf[i+2]
				} else {
					two = (15 * buf[i+2]) % M
				}
			}
			buf[i] = (one + two) % M
		} else {
			one, two := buf[i+1], 0
			if i < len(s)-1 && (s[i] == '1' || s[i] == '2') {
				if s[i] == '1' {
					if s[i+1] >= '0' && s[i+1] <= '9' {
						two = buf[i+2]
					} else {
						two = (9 * buf[i+2]) % M
					}
				} else if s[i] == '2' {
					if s[i+1] >= '0' && s[i+1] <= '6' {
						two = buf[i+2]
					} else if s[i+1] == '*' {
						two = (6 * buf[i+2]) % M
					}
				}
			}
			buf[i] = (one + two) % M
		}
	}
	return buf[0]
}

func main() {
	fmt.Println()
}
