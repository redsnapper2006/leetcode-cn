package main

import "fmt"

func findComplement(num int) int {
	num = ^num
	offset := 32
	for i := 31; i >= 0; i-- {
		if num&(1<<i) == 0 {
			offset = i
			break
		}
	}
	return num & (1<<(offset+1) - 1)
}

func main() {
	fmt.Println("a")
}
