package main

import "fmt"

func myPow(x float64, n int) float64 {
	if n == 0 {
		return 1
	}
	if n < 0 {
		x = 1 / x
		n = -n
	}

	t := x
	r := 1.0
	if 1&n > 0 {
		r = x
	}
	for i := 1; i < 32; i++ {
		t *= t
		if (1<<i)&n > 0 {
			r *= t
		}
	}
	return r
}

func main() {
	fmt.Println("a")
}
