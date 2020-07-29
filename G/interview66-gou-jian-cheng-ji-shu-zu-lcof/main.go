package main

import "fmt"

func constructArr(a []int) []int {
	left, right := make([]int, len(a)), make([]int, len(a))
	l, r := 1, 1

	for i := 0; i < len(a); i++ {
		l *= a[i]
		left[i] = l
		r *= a[len(a)-1-i]
		right[len(a)-1-i] = r
	}
	ret := make([]int, len(a))
	for i := 0; i < len(a); i++ {
		l, r := 1, 1
		if i > 0 {
			l = left[i-1]
		}
		if i < len(a)-1 {
			r = right[i+1]
		}
		ret[i] = l * r
	}
	return ret
}

func main() {
	fmt.Println("a")
}
