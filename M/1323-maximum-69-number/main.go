package main

import "fmt"

func maximum69Number(num int) int {
	var buf []int

	for num > 0 {
		n := num % 10
		buf = append(buf, n)
		num /= 10
	}
	s, e := 0, len(buf)-1
	for s < e {
		buf[s], buf[e] = buf[e], buf[s]
		s++
		e--
	}
	for i := 0; i < len(buf); i++ {
		if buf[i] == 6 {
			buf[i] = 9
			break
		}
	}
	ret := 0
	for i := 0; i < len(buf); i++ {
		ret = ret*10 + buf[i]
	}
	return ret
}

func main() {
	fmt.Println("a")
}
