package main

import "fmt"

func cuttingRope(n int) int {
	if n <= 3 {
		return n - 1
	}
	b := n % 3
	rem := 1
	x := 3
	for a := n/3 - 1; a > 0; a /= 2 {
		if a%2 == 1 {
			rem = (rem * x) % 1000000007
		}
		x = (x * x) % 1000000007
	}

	if b == 0 {
		return rem * 3 % 1000000007
	}
	if b == 1 {
		return rem * 4 % 1000000007
	}
	return rem * 6 % 1000000007
}

func main() {
	fmt.Println("a")
}
