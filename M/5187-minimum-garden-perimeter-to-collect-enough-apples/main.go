package main

import (
	"fmt"
)

func minimumPerimeter(neededApples int64) int64 {
	var sum int64
	sum = 0
	var i int64
	i = 0
	for sum < neededApples {
		i++
		sum += 4*(i*(i+1)+i*(2*i+1)) - 4*2*i
	}

	return i * 8
}

func main() {
	fmt.Println()
}
