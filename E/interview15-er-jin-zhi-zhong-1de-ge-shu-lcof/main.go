package main

import "fmt"

func hammingWeight(num uint32) int {
	c := 0
	for {
		if num%2 == 1 {
			c++
		}
		if num < 2 {
			break
		}
		num = num / 2
	}
	return c
}

func main() {
	fmt.Println("a")
}
