package main

import "fmt"

func divide(dividend int, divisor int) int {
	if dividend == 0 {
		return 0
	}

	if dividend == 1<<31-1 {
		if divisor == 1 {
			return 1<<31 - 1
		} else if divisor == -1 {
			return -1<<31 + 1
		}
	}
	if dividend == -1<<31 {
		if divisor == 1 {
			return -1 << 31
		} else if divisor == -1 {
			return 1<<31 - 1
		}
	}

	isMinus := false
	if (dividend > 0 && divisor < 0) || (dividend < 0 && divisor > 0) {
		isMinus = true
	}

	ABS := func(n int) int {
		// y := n >> 63
		y := n >> 31
		return (n ^ y) - y
	}

	if dividend < 0 {
		dividend = ABS(dividend)
	}
	if divisor < 0 {
		divisor = ABS(divisor)
	}

	times := 0
	extra := 0
	for i := 31; i >= 0; i-- {
		if (extra + (divisor << i)) <= dividend {
			extra += (divisor << i)
			times |= 1 << i
		}
	}

	if isMinus {
		// n:=0
		// for i:=0;i<times;i++ {
		// 	n--
		// }
		times = -times
	}

	return times
}

func main() {
	fmt.Println(divide(10, 3))
	fmt.Println(divide(7, -3))
	fmt.Println(divide(-7, 3))
	fmt.Println(divide(-7, -3))
	fmt.Println(divide(-2147483648, 2))
}
