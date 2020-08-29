package main

import "fmt"

func thousandSeparator(n int) string {
	if n == 0 {
		return "0"
	}
	var buf []byte
	c := 0
	for n > 0 {
		t := n % 10
		buf = append(buf, byte('0'+t))
		c++
		if c == 3 {
			buf = append(buf, '.')
			c = 0
		}
		n /= 10
	}
	if buf[len(buf)-1] == '.' {
		buf = buf[0 : len(buf)-1]
	}
	s, e := 0, len(buf)-1
	for s < e {
		buf[s], buf[e] = buf[e], buf[s]
		s++
		e--
	}
	return string(buf)
}

func main() {
	fmt.Println("a")
}
