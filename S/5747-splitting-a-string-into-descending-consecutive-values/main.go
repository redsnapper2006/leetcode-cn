package main

import "fmt"

func splitString(s string) bool {
	var recur func(buf []byte, prev int) bool
	recur = func(buf []byte, prev int) bool {
		if len(buf) == 0 {
			return true
		}
		base := 0
		idx := -1
		for i := 0; i < len(buf); i++ {
			base = base*10 + int(buf[i]-byte('0'))
			if base >= prev {
				return false
			}
			// fmt.Println("recur", prev, base)
			if prev-base == 1 {
				idx = i
				r := recur(buf[idx+1:], base)
				if r {
					return r
				}
			}
		}
		return false
	}

	buf := []byte(s)
	base := 0
	for i := 0; i < len(buf)-1; i++ {
		base = base*10 + int(buf[i]-byte('0'))
		// fmt.Println("first", base)
		r := recur(buf[i+1:], base)
		if r {
			return r
		}
	}
	return false
}

func main() {
	fmt.Println(splitString("200100"))
}
