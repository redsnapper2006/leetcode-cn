package main

import (
	"fmt"
	"strconv"
)

func fractionToDecimal(numerator int, denominator int) string {
	s := ""
	n, d := numerator, denominator

	if (n > 0 && d < 0) || (n < 0 && d > 0) {
		s += "-"
	}

	if n < 0 {
		n *= -1
	}
	if d < 0 {
		d *= -1
	}

	s += strconv.Itoa(n / d)
	if n%d == 0 {
		return s
	}

	s += "."

	n = n % d
	n *= 10
	for n < d {
		s += "0"
		n *= 10
	}

	modMap := make(map[int]int)
	var buf []int

	buf = append(buf, n/d)
	modMap[n%d] = 1
	n %= d

	isLoop := false
	start := -1
	for n%d != 0 {
		n *= 10
		buf = append(buf, n/d)
		idx, ok := modMap[n%d]
		if !ok {
			modMap[n%d] = len(buf)
		} else {
			isLoop = true
			start = idx
			break
		}
		n %= d
	}

	for i := 0; i < len(buf); i++ {
		if i == start && isLoop {
			s += "("
		}
		s += strconv.Itoa(buf[i])
	}

	if isLoop {
		s += ")"
	}
	return s
}

func main() {
	// fmt.Println(fractionToDecimal(1, 2))
	// fmt.Println(fractionToDecimal(2, 1))
	// fmt.Println(fractionToDecimal(2, 3))
	fmt.Println(fractionToDecimal(1, 6))
	fmt.Println(fractionToDecimal(-50, 8))
	fmt.Println(fractionToDecimal(7, -12))
	fmt.Println(fractionToDecimal(-2147483648, -10))

}
