package main

import "fmt"

func numWaterBottles(numBottles int, numExchange int) int {
	total := numBottles
	for numBottles >= numExchange {
		r := numBottles / numExchange
		m := numBottles % numExchange
		total += r
		numBottles = r + m
	}
	return total
}

func main() {
	fmt.Println("a")
}
