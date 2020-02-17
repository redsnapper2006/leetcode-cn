package main

import (
	"fmt"
)

func monotoneIncreasingDigits(N int) int {
	if N < 10 {
		return N
	}
	n := N
	var buf []int
	for n >= 10 {
		buf = append(buf, n%10)
		n /= 10
	}
	buf = append(buf, n)
	fmt.Println(buf)
	sIdx := len(buf) - 1
	for i := len(buf) - 1; i >= 1; i-- {
		if buf[i] < buf[i-1] {
			sIdx = i - 1
		} else if buf[i] > buf[i-1] {
			break
		}
	}
	fmt.Println(sIdx)

	if sIdx == 0 {
		return N
	}

	a := 0
	for i := len(buf) - 1; i > sIdx; i-- {
		a = a*10 + buf[i]
	}
	a = a*10 + (buf[sIdx] - 1)

	for i := 0; i < sIdx; i++ {
		a = a*10 + 9
	}
	return a
}

func main() {
	fmt.Println(monotoneIncreasingDigits(10))
	fmt.Println(monotoneIncreasingDigits(332))
	fmt.Println(monotoneIncreasingDigits(1234))
	fmt.Println(monotoneIncreasingDigits(12334345431))
}
