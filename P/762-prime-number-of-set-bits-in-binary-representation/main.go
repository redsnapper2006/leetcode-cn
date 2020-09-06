package main

import (
	"fmt"
)

func countPrimeSetBits(L int, R int) int {
	f := make([]bool, 20)
	for i := 0; i < 20; i++ {
		f[i] = true
	}
	f[2] = false
	f[3] = false
	f[5] = false
	f[7] = false
	f[11] = false
	f[13] = false
	f[17] = false
	f[19] = false

	r := 0
	for i := L; i <= R; i++ {
		c := 0
		for b := 20; b >= 0; b-- {
			if i&(1<<b) > 0 {
				c++
			}
		}
		if f[c] == false {
			r++
		}
	}
	return r
}

func main() {
	fmt.Println("a")
}
