package main

import "fmt"

func exchangeBits(num int) int {
	// even := 0xaaaaaaaa
	// odd := 0x55555555
	return ((num & 0xaaaaaaaa) >> 1) | ((num & 0x55555555) << 1)
}

func main() {
	fmt.Println("a")
}
