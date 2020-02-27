package main

import "fmt"

func numPairsDivisibleBy60(time []int) int {
	buf := make(map[int]int)
	for i := 0; i < len(time); i++ {
		buf[time[i]%60]++
	}

	c := 0
	for i := 0; i < len(time); i++ {
		k1 := time[i] % 60
		buf[k1]--
		k := (60 - k1) % 60
		c += buf[k]
	}
	return c
}

func main() {
	fmt.Println("a")
}
