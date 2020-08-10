package main

import "fmt"

func countBinarySubstrings(s string) int {
	var buf []int
	b := s[0]
	c := 1
	for i := 1; i < len(s); i++ {
		if s[i] == b {
			c++
		} else {
			b = s[i]
			buf = append(buf, c)
			c = 1
		}
	}
	buf = append(buf, c)
	r := 0
	for i := 0; i < len(buf)-1; i++ {
		min := buf[i]
		if min > buf[i+1] {
			min = buf[i+1]
		}
		r += min
	}
	return r
}

func main() {
	fmt.Println("a")
}
