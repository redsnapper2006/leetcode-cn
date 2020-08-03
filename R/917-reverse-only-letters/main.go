package main

import "fmt"

func reverseOnlyLetters(S string) string {
	buf := []byte(S)
	s, e := 0, len(S)-1
	for {
		for s < len(S) && !((buf[s] >= 'a' && buf[s] <= 'z') || (buf[s] >= 'A' && buf[s] <= 'Z')) {
			s++
		}
		for e >= 0 && !((buf[e] >= 'a' && buf[e] <= 'z') || (buf[e] >= 'A' && buf[e] <= 'Z')) {
			e--
		}

		if s > e {
			break
		}

		buf[s], buf[e] = buf[e], buf[s]

		s++
		e--
	}
	return string(buf)
}

func main() {
	fmt.Println(reverseOnlyLetters("?6C40E"))
}
