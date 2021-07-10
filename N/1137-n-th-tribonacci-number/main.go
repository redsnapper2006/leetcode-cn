package main

import "fmt"

func tribonacci(n int) int {
	t0, t1, t2, t3 := 0, 1, 1, 2

	for i := 3; i < n+1; i++ {
		if i%4 == 3 {
			t3 = t0 + t1 + t2
		}
		if i%4 == 0 {
			t0 = t1 + t2 + t3
		}
		if i%4 == 1 {
			t1 = t2 + t3 + t0
		}
		if i%4 == 2 {
			t2 = t3 + t0 + t1
		}
	}

	if n%4 == 0 {
		return t0
	}
	if n%4 == 1 {
		return t1
	}
	if n%4 == 2 {
		return t2
	}
	if n%4 == 3 {
		return t3
	}
	return -1
}

func main() {
	fmt.Println("a")
}
