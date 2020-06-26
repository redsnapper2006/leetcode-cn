package main

import "fmt"

func convertInteger(A int, B int) int {
	r := A ^ B
	c := 0
	for i := 0; i < 32; i++ {
		if r&(1<<i) > 0 {
			c++
		}
	}
	return c
}

func main() {
	fmt.Println("a")
}
