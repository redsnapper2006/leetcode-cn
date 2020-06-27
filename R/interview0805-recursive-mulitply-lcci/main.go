package main

import "fmt"

func multiply(A int, B int) int {
	var recur func(A, B, C, R int) int
	recur = func(A, B, C, R int) int {
		if C == -1 {
			return R
		}
		if B&(1<<C) > 0 {
			R += A << C
		}
		return recur(A, B, C-1, R)
	}
	return recur(A, B, 31, 0)
}

func main() {
	fmt.Println("a")
}
