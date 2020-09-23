package main

import (
	"fmt"
	"math"
)

func numPrimeArrangements(n int) int {

	sieveOfEratosthenes := func(value int) int {
		f := make([]bool, value)
		for i := 2; i <= int(math.Sqrt(float64(value))); i++ {
			if f[i] == false {
				for j := i * i; j < value; j += i {
					f[j] = true
				}
			}
		}
		c := 0
		for i := 2; i < value; i++ {
			if f[i] == false {
				c++
			}
		}
		return c
	}
	count := sieveOfEratosthenes(n + 1)
	ret := 1
	for i := 1; i <= count; i++ {
		ret *= i
		ret %= 1000000007
	}
	for i := 1; i <= n-count; i++ {
		ret *= i
		ret %= 1000000007
	}
	return ret
}

func main() {
	fmt.Println("a")
}
