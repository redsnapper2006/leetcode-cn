package main

import "fmt"

func integerReplacement(n int) int {
	var recur func(n int) int
	recur = func(n int) int {
		if n == 1 {
			return 0
		}
		if n == 2 {
			return 1
		}
		if n%2 == 0 {
			return recur(n/2) + 1
		} else {
			t1 := recur((n+1)/2) + 2
			t2 := recur((n-1)/2) + 2
			if t1 > t2 {
				return t2
			} else {
				return t1
			}
		}
	}
	return recur(n)
}

func main() {
	fmt.Println("a")
}
