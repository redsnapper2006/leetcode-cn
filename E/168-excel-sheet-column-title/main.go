package main

import "fmt"

func convertToTitle(n int) string {
	var buf []byte
	for n > 0 {
		m := n % 26
		if m == 0 {
			m = 26
		}
		buf = append(buf, byte(m-1+int('A')))
		n--
		n = n / 26
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
	fmt.Println(convertToTitle(52))
}
