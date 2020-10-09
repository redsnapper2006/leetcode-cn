package main

import "fmt"

func prefixesDivBy5(A []int) []bool {
	var ret []bool
	sum := 0
	for _, v := range A {
		sum = sum*2 + v
		b := false
		if sum%5 == 0 {
			b = true
		}
		sum %= 5
		ret = append(ret, b)
	}
	return ret
}

func main() {
	fmt.Println("a")
}
