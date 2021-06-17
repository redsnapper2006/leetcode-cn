package main

import (
	"fmt"
)

func addDigits(num int) int {
	if num < 10 {
		return num
	}
	if num%9 == 0 {
		return 9
	}
	return num % 9
}

func main() {
	fmt.Println("a")
}
