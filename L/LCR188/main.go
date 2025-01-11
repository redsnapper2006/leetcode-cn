package main

import "fmt"

func maxProfit(prices []int) int {
	if len(prices) <= 1 {
		return 0
	}

	min, max := prices[0], prices[0]
	r := 0
	for i := 1; i < len(prices); i++ {
		if prices[i] < min {
			if max-min > r {
				r = max - min
			}
			min, max = prices[i], prices[i]
		} else if prices[i] > max {
			max = prices[i]
		}
	}
	if max-min > r {
		r = max - min
	}
	return r
}

func main() {
	fmt.Println("a")
}
