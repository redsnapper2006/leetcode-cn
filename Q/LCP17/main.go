package main

import "fmt"

func calculate(s string) int {
	x, y := 1, 0
	for _, b := range s {
		if b == 'A' {
			x = 2*x + y
		} else {
			y = 2*y + x
		}
	}
	return x + y
}

func main() {
	fmt.Println()
}
