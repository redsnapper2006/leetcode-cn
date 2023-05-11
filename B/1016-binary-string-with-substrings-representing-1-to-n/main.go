package main

import "fmt"

func queryString(s string, n int) bool {
	dp := make([][]int, len(s))
	for idx, b := range s {
		dp[idx] = make([]int, 31)
		single := int(b - '0')
		dp[idx][0] = single
		if idx > 0 {
			for i := 1; i < 31; i++ {
				dp[idx][i] = dp[idx-1][i-1]*2 + single
			}
		}
	}

	buf := map[int]int{}
	for _, v := range dp {
		for _, b := range v {
			if b <= n {
				buf[b] = 1
			}
		}
	}
	start := 1
	for start <= n {
		if _, ok := buf[start]; !ok {
			return false
		}
		start++
	}
	return true
}

func main() {
	fmt.Println(queryString("0110", 4))
	fmt.Println(queryString("1111000101", 5))
	fmt.Println(queryString("011010101010111101010101011111111111111111111111111111111110000000000000011111101010101001010101010101010101010101111010101010111111111111111111111111111111111100000000000000111111010101010010101010101010101010100", 1000000000))

}
