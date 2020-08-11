package main

import "fmt"

func fraction(cont []int) []int {
	divisor := cont[len(cont)-1]
	divide := 1
	for i := len(cont) - 2; i >= 0; i-- {
		divisor, divide = cont[i]*divisor+divide, divisor
	}

	return []int{divisor, divide}
}

func main() {
	fmt.Println("a")
}
