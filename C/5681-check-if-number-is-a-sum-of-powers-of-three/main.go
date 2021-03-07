package main

import "fmt"

func checkPowersOfThree(n int) bool {
	m := n
	steps := 0
	M := map[int]int{}
	for m > 0 {
		if m%3 != 0 {
			m--
			_, ok := M[steps]
			if ok {
				return false
			} else {
				M[steps]++
			}
		} else {
			m /= 3
			steps++
		}
	}
	return true
}

func main() {
	fmt.Println("")
}
