package main

import (
	"fmt"
)

func findNthDigit(n int) int {
	times := 9
	digit := 1
	bitCount := 9

	// 1-9 10-99 100-999
	for n > bitCount {
		times *= 10
		digit++
		bitCount += times * digit
	}
	bitCount -= times * digit
	n -= bitCount
	offset1 := (n - 1) / digit
	offset2 := (n - 1) % digit
	start := 1
	for i := 0; i < digit-1; i++ {
		start *= 10
	}
	w := start + offset1
	m := start + offset1
	for i := 0; i < digit-offset2; i++ {
		m = w % 10
		w /= 10
	}
	return m
}

func main() {
	fmt.Println("a")
}
