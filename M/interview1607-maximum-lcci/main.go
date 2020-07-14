package main

import "fmt"

func maximum(a int, b int) int {
	x := a - b
	k := x >> 32
	return (1+k)*a - b*k
}

func main() {
	fmt.Println("a")
}
