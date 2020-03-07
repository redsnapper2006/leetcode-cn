package main

import "fmt"

func myPowRecur(x float64, n int) float64 {
	if n == 0 {
		return 1
	}
	if n < 0 {
		x = 1 / x
		n = -n
	}
	var recur func(x float64, n int) float64
	recur = func(x float64, n int) float64 {
		if n == 1 {
			return x
		}
		t := recur(x, n/2)
		if n%2 == 0 {
			return t * t
		} else {
			return t * t * x
		}
	}
	return recur(x, n)
}

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
