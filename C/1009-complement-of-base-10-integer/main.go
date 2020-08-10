package main

import "fmt"

func bitwiseComplement(N int) int {
	if N == 0 {
		return 1
	}
	var buf []int
	for N > 0 {
		buf = append(buf, N%2)
		N /= 2
	}

	r := 0
	for i := len(buf) - 1; i >= 0; i-- {
		r = r*2 + (1 - buf[i])
	}
	return r
}

func main() {
	fmt.Println("a")
}
