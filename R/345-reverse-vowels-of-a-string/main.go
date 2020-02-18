package main

import (
	"fmt"
)

func reverseVowels(str string) string {
	s, e := 0, len(str)-1
	buf := make([]byte, len(str))
	for s < e {
		for s < e {
			if str[s] != 'a' && str[s] != 'e' && str[s] != 'i' && str[s] != 'o' && str[s] != 'u' &&
				str[s] != 'A' && str[s] != 'E' && str[s] != 'I' && str[s] != 'O' && str[s] != 'U' {
				buf[s] = str[s]
				s++
			} else {
				break
			}
		}
		for e > s {
			if str[e] != 'a' && str[e] != 'e' && str[e] != 'i' && str[e] != 'o' && str[e] != 'u' &&
				str[e] != 'A' && str[e] != 'E' && str[e] != 'I' && str[e] != 'O' && str[e] != 'U' {
				buf[e] = str[e]
				e--
			} else {
				break
			}
		}
		buf[s] = str[e]
		buf[e] = str[s]
		s++
		e--
	}
	if s == e {
		buf[s] = str[s]
	}
	return string(buf)
}

func main() {
	fmt.Println(reverseVowels("hello"))
}
