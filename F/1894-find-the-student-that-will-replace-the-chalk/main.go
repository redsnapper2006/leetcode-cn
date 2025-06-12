package main

import "fmt"

func chalkReplacer(chalk []int, k int) int {
	sum := make([]int, len(chalk))
	sum[0] = chalk[0]
	for i := 1; i < len(chalk); i++ {
		sum[i] = sum[i-1] + chalk[i]
	}
	k %= sum[len(sum)-1]
	for i := 0; i < len(sum); i++ {
		if sum[i] > k {
			return i
		}
	}
	return -1
}

func main() {
	fmt.Println()
}
