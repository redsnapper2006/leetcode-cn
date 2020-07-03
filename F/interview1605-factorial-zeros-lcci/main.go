package main

import (
	"fmt"
)

func trailingZeroes(n int) int {
	if n < 5 {
		return 0
	}

	table := []int{5, 25, 125, 625, 3125, 15625, 78125, 390625, 1953125, 9765625, 48828125, 244140625, 1220703125}
	c := 0
	for i := 0; i < len(table); i++ {
		if n >= table[i] {
			c += n / table[i]
		} else {
			break
		}
	}
	return c
}

func main() {
	fmt.Println(trailingZeroes(3))
	fmt.Println(trailingZeroes(5))
	fmt.Println(trailingZeroes(6))
	fmt.Println(trailingZeroes(10))
	fmt.Println(trailingZeroes(12))
	fmt.Println(trailingZeroes(15))
	fmt.Println(trailingZeroes(25))
	fmt.Println(trailingZeroes(50))
	fmt.Println(trailingZeroes(200))
	fmt.Println(trailingZeroes(1808548329))
	fmt.Println(trailingZeroes(2147483647))
}
