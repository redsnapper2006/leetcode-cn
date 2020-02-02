package main

import (
	"fmt"
)

func grayCode(n int) []int {
	if n == 0 {
		return []int{0}
	}

	if n == 1 {
		return []int{0, 1}
	}
	var r []int
	c := grayCode(n - 1)
	for i := 0; i < len(c); i++ {
		r = append(r, c[i]*2+0)
	}
	for i := len(c) - 1; i >= 0; i-- {
		r = append(r, c[i]*2+1)
	}
	return r
}

func main() {
	fmt.Println(grayCode(0))
	fmt.Println(grayCode(1))
	fmt.Println(grayCode(2))
	fmt.Println(grayCode(3))
	fmt.Println(grayCode(5))
}
