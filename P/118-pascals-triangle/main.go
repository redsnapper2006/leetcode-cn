package main

import (
	"fmt"
)

func factor(n, i int) int {
	if i == 0 {
		return 1
	}
	if i == 1 {
		return n
	}
	s, c := 1, n
	for j := 0; j < i; j++ {
		s *= c
		c--
	}
	c = i
	for j := 0; j < i; j++ {
		s /= c
		c--
	}
	return s
}

func yhsj(n int) []int {
	if n == 1 {
		return []int{1}
	}
	var buf []int
	for i := 0; i < (n+1)/2; i++ {
		buf = append(buf, factor(n-1, i))
	}
	for i := (n + 1) / 2; i < n; i++ {
		buf = append(buf, buf[n-i-1])
	}
	return buf
}

func generate(numRows int) [][]int {
	var ret [][]int
	for i := 0; i < numRows; i++ {
		ret = append(ret, yhsj(i+1))
	}
	return ret
}

func main() {
	fmt.Println(generate(10))
}
