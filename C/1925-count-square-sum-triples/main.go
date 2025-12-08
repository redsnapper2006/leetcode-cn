package main

import (
	"math"
	"fmt"
)

func countTriples(n int) int {
	ret := 0
	for i := 2; i <= n; i++ {
		for j := 1; j < i; j++ {
			diff := i*i - j*j
			// fmt.Println("start",i,j)
			v := int(math.Sqrt(float64(diff)))
			// fmt.Println(diff, v, v*v == diff)
			if v*v == diff {
				ret++
			}
		}
	}
	return ret
}

func main() {
	fmt.Println(countTriples(5))
	fmt.Println(countTriples(10))
}
