package main

import (
	"fmt"
	"math"
)

func sieveOfEratosthenes(value int) int {
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

func countPrimes(n int) int {
	return sieveOfEratosthenes(n)
}

func main() {
	fmt.Println(countPrimes(10))
}
