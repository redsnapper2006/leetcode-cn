package main

import "fmt"

func add(a int, b int) int {
	return (a ^ b) + ((a & b) << 1)
}

func main() {
	fmt.Println(add(10, 1))
	fmt.Println(^1)
}
