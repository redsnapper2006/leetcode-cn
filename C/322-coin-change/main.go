package main

import (
	"fmt"
)

func coinChange(coins []int, amount int) int {
	buf := make([]int, amount+1)

	buf[amount] = 1
	for i := amount; i >= 1; i-- {
		if buf[i] == 0 {
			continue
		}
		for j := 0; j < len(coins); j++ {
			if i >= coins[j] && ((buf[i-coins[j]] > 0 && buf[i-coins[j]] > buf[i]+1) || buf[i-coins[j]] == 0) {
				buf[i-coins[j]] = buf[i] + 1
			}
		}
	}
	return buf[0] - 1
}

func main() {
	fmt.Println(coinChange([]int{1, 2, 5}, 11))
	fmt.Println(coinChange([]int{2}, 3))
}
