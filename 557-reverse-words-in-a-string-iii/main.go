package main

import (
	"fmt"
)

func reverseWords(s string) string {
	if len(s) == 0 {
		return s
	}

	buf := []byte{}
	startOffset := -1
	for i := 0; i < len(s); i++ {
		if s[i] == ' ' {
			size := i - startOffset - 1
			if size > 0 {
				s, e := startOffset+1, i-1
				for s < e {
					t := buf[s]
					buf[s] = buf[e]
					buf[e] = t
					s++
					e--
				}
			}
			startOffset = i
		}
		buf = append(buf, s[i])
	}
	size := len(s) - startOffset - 1
	if size > 0 {
		s, e := startOffset+1, len(s)-1
		for s < e {
			t := buf[s]
			buf[s] = buf[e]
			buf[e] = t
			s++
			e--
		}
		buf = append(buf, ' ')
	}
	buf = buf[0 : len(buf)-1]
	// fmt.Println("["+string(buf)+"]", len(string(buf)))
	return string(buf)
}

func main() {
	fmt.Println(reverseWords("Let's take LeetCode contest"))
}
