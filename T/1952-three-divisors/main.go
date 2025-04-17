package main

import (
	"fmt"
	"math"
)

func isThree(n int) bool {
	if n <= 2 {
		return false
	}
	for i := 2; i < int(math.Sqrt(float64(n))); i++ {
		if n%i == 0 {
			return false
		}
	}
	d := int(math.Sqrt(float64(n)))
	if d*d == n {
		return true
	}
	return false
}

func main() {
	fmt.Println()
}
