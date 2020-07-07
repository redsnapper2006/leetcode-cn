package main

import (
	"fmt"
	"sort"
)

func change(amount int, coins []int) int {
	if len(coins) == 0 {
		if amount == 0 {
			return 1
		} else {
			return 0
		}
	}
	sort.Ints(coins)
	buf := make([][]int, len(coins))
	for i := 0; i < len(buf); i++ {
		buf[i] = make([]int, amount+1)
	}
	buf[0][0] = 1
	for i := 0; i <= amount; i++ {
		for j := 0; j < len(coins); j++ {
			if buf[j][i] == 0 {
				continue
			}
			for m := j; m < len(coins); m++ {
				if i+coins[m] > amount {
					continue
				}
				buf[m][i+coins[m]] += buf[j][i]
			}
		}
	}
	fmt.Println(buf)
	c := 0
	for i := 0; i < len(coins); i++ {
		c += buf[i][amount]
	}
	return c
}

func main() {
	fmt.Println("a")
}
