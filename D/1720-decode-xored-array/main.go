package main

import "fmt"

func decode(encoded []int, first int) []int {
	base := first
	ret := []int{base}
	for _, b := range encoded {
		base ^= b
		ret = append(ret, base)
	}
	return ret
}

func main() {
	fmt.Println()
}
