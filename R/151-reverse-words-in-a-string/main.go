package main

import (
	"fmt"
)

func reverseWords(s string) string {
	if len(s) == 0 {
		return s
	}

	end := len(s) - 1
	for end >= 0 && s[end] == ' ' {
		end--
	}
	if end < 0 {
		return ""
	}

	buf := []byte{}
	endOffset := end + 1
	for i := end; i >= 0; i-- {
		if s[i] == ' ' {
			size := endOffset - i - 1
			if size > 0 {
				s, e := len(buf)-size, len(buf)-1
				for s < e {
					t := buf[s]
					buf[s] = buf[e]
					buf[e] = t
					s++
					e--
				}
				buf = append(buf, ' ')
			}
			endOffset = i
			continue
		}
		buf = append(buf, s[i])
	}
	size := endOffset
	if size > 0 {
		s, e := len(buf)-size, len(buf)-1
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
	fmt.Println(reverseWords("the sky is blue"))
	fmt.Println(reverseWords("  hello world!  "))
	fmt.Println(reverseWords("ab good   example"))
	fmt.Println(reverseWords("       "))
}
