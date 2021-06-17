package main

import (
	"fmt"
)

func strWithout3a3b(A int, B int) string {
	min := A
	max := B
	b := 'a'
	n := 'b'
	if min > B {
		min = B
		max = A
		b = 'b'
		n = 'a'
	}
	if max > 2*min {
		var s []byte
		for i := 0; i < min; i++ {
			s = append(s, byte(n), byte(n), byte(b))
		}
		for i := 0; i < max-2*min; i++ {
			s = append(s, byte(n))
		}
		return string(s)
	}
	var s []byte
	for i := 0; i < max-min; i++ {
		s = append(s, byte(n), byte(n), byte(b))
	}
	for i := 0; i < 2*min-max; i++ {
		s = append(s, byte(n), byte(b))
	}
	return string(s)
}

func main() {
	fmt.Println("a")
}
