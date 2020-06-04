package main

import "fmt"

func hasAlternatingBits(n int) bool {
	idx := 31
	for idx >= 0 {
		if n&(1<<idx) > 0 {
			break
		}
		idx--
	}
	isOne := true
	for i := idx; i >= 0; i-- {
		t := n & (1 << i)
		if (isOne && t > 0) || (!isOne && t == 0) {
			isOne = !isOne
		} else {
			return false
		}
	}
	return true
}

func main() {
	fmt.Println("a")
}
