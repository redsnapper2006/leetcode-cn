package main

import "fmt"

func brokenCalc(X int, Y int) int {
	steps := 0
	for {
		if X >= Y {
			return steps + X - Y
		}

		if Y%2 == 1 {
			Y++
			steps++
		}
		Y /= 2
		steps++
	}
}

func main() {
	fmt.Println(brokenCalc(5, 8))
}
