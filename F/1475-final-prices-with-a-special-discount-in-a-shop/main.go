package main

import "fmt"

func finalPrices(prices []int) []int {

	buf := make([]int, len(prices))
	for i := 0; i < len(prices); i++ {
		discount := 0
		for j := i + 1; j < len(prices); j++ {
			if prices[j] <= prices[i] {
				discount = prices[j]
				break
			}
		}
		buf[i] = prices[i] - discount
	}
	return buf
}

func main() {
	fmt.Println("a")
}
