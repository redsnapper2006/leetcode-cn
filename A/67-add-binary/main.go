package main

import (
	"fmt"
)

func addBinary(a string, b string) string {
	var base, loop string
	if len(a) > len(b) {
		base = a
		loop = b
	} else {
		base = b
		loop = a
	}

	var buf []byte
	isPlus := false
	for i := 0; i < len(loop); i++ {
		if loop[len(loop)-i-1] == '0' && base[len(base)-i-1] == '0' {
			if isPlus {
				buf = append(buf, '1')
			} else {
				buf = append(buf, '0')
			}
			isPlus = false
		} else if (loop[len(loop)-i-1] == '1' && base[len(base)-i-1] == '0') || (loop[len(loop)-i-1] == '0' && base[len(base)-i-1] == '1') {
			if isPlus {
				buf = append(buf, '0')
				isPlus = true
			} else {
				buf = append(buf, '1')
				isPlus = false
			}
		} else {
			if isPlus {
				buf = append(buf, '1')
			} else {
				buf = append(buf, '0')
			}
			isPlus = true
		}
	}

	for i := len(loop); i < len(base); i++ {
		if isPlus {
			if base[len(base)-i-1] == '0' {
				buf = append(buf, '1')
				isPlus = false
			} else {
				buf = append(buf, '0')
				isPlus = true
			}
		} else {
			buf = append(buf, base[len(base)-i-1])
		}
	}
	if isPlus {
		buf = append(buf, '1')
	}
	s := ""
	for _, v := range buf {
		s = string(v) + s
	}
	return s
}

func main() {
	fmt.Println(addBinary("11100", "110010"))
}
