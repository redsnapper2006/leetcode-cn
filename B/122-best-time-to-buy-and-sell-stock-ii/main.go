package main

import "fmt"

func maxProfit(prices []int) int {
	if len(prices) <= 1 {
		return 0
	}

	min, max := prices[0], prices[0]
	sum := 0
	for i := 1; i < len(prices); i++ {
		if prices[i] >= max {
			max = prices[i]
		} else if prices[i] < max {
			sum += max - min
			min, max = prices[i], prices[i]
		}
	}
	sum += max - min

	return sum
}

func main() {
	fmt.Println("a")
}
