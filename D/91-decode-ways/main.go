package main

import (
	"fmt"
	"strconv"
)

func numDecodings(s string) int {
	buf := make([]int, len(s)+2)
	buf[len(s)] = 1
	buf[len(s)+1] = 0
	for i := len(s) - 1; i >= 0; i-- {
		if s[i] == '0' {
			buf[i] = 0
		} else {
			one, two := 0, 0
			v1, _ := strconv.ParseInt(s[i:i+1], 10, 32)
			if v1 > 0 && v1 < 10 {
				one = buf[i+1]
			}
			if i < len(s)-1 {
				v2, _ := strconv.ParseInt(s[i:i+2], 10, 32)
				if v2 > 9 && v2 < 27 {
					two = buf[i+2]
				}
			}
			buf[i] = one + two
		}
	}
	return buf[0]
}

func main() {
	fmt.Println("a")
}
