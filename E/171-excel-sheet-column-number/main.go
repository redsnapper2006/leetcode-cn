package main

import (
	"fmt"
)

const (
	NUM = 26
)

func titleToNumber(s string) int {
	ret := 0
	for _, v := range s {
		ret = ret*NUM + int(v-'A') + 1
	}
	return ret
}

func main() {
	fmt.Println(titleToNumber("A"))
	fmt.Println(titleToNumber("B"))
	fmt.Println(titleToNumber("Z"))
	fmt.Println(titleToNumber("AB"))
	fmt.Println(titleToNumber("ZY"))
}
