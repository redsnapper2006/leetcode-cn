package main

import "fmt"

func coinChange(coins []int, amount int) int {
	buf := make([]int, amount+1)
	buf[0] = 1
	for i := 0; i <= amount; i++ {
		if buf[i] == 0 {
			continue
		}
		for _, v := range coins {
			if i+v <= amount && (buf[i+v] == 0 || buf[i+v] > buf[i]+1) {
				buf[i+v] = buf[i] + 1
			}
		}
	}
	return buf[amount] - 1
}

func main() {
	fmt.Println()
}
