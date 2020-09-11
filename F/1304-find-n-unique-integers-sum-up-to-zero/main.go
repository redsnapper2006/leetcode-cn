package main

import "fmt"

func sumZero(n int) []int {
	var ret []int
	isSkip := false
	if n%2 == 0 {
		isSkip = true
	} else {
		isSkip = false
	}
	for i := -(n / 2); i <= n/2; i++ {
		if i == 0 && isSkip {
			continue
		}
		ret = append(ret, i)
	}
	return ret
}

func main() {
	fmt.Println("a")
}
