package main

import (
	"fmt"
)

func minimumSwap(s1 string, s2 string) int {
	x, y := 0, 0
	for i := 0; i < len(s1); i++ {
		if byte(s1[i]) != byte(s2[i]) {
			if byte(s1[i]) == 'x' {
				x++
			} else {
				y++
			}
		}
	}
	if (x+y)%2 != 0 {
		return -1
	}
	ret := 0
	ret += x / 2
	ret += y / 2
	if x%2 > 0 {
		ret += 2
	}
	return ret
}

func main() {
	fmt.Println(minimumSwap("xx", "yy"))
	fmt.Println(minimumSwap("xy", "yx"))
}
