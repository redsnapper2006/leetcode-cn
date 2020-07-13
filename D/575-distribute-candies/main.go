package main

import "fmt"

func distributeCandies(candies []int) int {
	M := map[int]int{}
	for i := 0; i < len(candies); i++ {
		M[candies[i]]++
	}
	r := len(candies) / 2
	if r > len(M) {
		r = len(M)
	}
	return r
}

func main() {
	fmt.Println("a")
}
