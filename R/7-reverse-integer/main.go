package main

import (
	"fmt"
)

func reverse(x int) int {
	var isMinus bool
	b := x
	if x < 0 {
		isMinus = true
		b *= -1
	} else {
		isMinus = false
	}
	r := 0
	for b/10 > 0 {
		if 2147483647-r*10 < b%10 {
			return 0
		}
		r = r*10 + b%10
		b /= 10
	}

	if 2147483647-r*10 < b {
		return 0
	}
	r = r*10 + b
	if isMinus {
		return r * -1
	} else {
		return r
	}
}

func main() {
	fmt.Println(reverse(1534236469))
	fmt.Println(reverse(123))
}
