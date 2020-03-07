package main

import "fmt"

func hammingDistance(x int, y int) int {
	c := 0
	for i := 0; i < 32; i++ {
		b1, b2 := (x>>i)&1, (y>>i)&1
		if b1 != b2 {
			c++
		}
	}
	return c
}

func main() {
	fmt.Println(hammingDistance(1, 4))
}
