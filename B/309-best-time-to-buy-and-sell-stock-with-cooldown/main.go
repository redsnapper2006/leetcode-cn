package main

import "fmt"

func maxProfit(prices []int) int {
	if len(prices) <= 1 {
		return 0
	}

	max := func(a, b int) int {
		r := a
		if r < b {
			r = b
		}
		return r
	}
	buf := make([][]int, 3)
	for i := 0; i < 3; i++ {
		buf[i] = make([]int, len(prices))
	}
	buf[0][0] = -prices[0]
	for i := 1; i < len(prices); i++ {
		buf[0][i] = max(buf[0][i-1], buf[2][i-1]-prices[i])
		buf[1][i] = buf[0][i-1] + prices[i]
		buf[2][i] = max(buf[2][i-1], buf[1][i-1])
	}
	return max(buf[1][len(prices)-1], buf[2][len(prices)-1])
}

func main() {
	fmt.Println("a")
}
