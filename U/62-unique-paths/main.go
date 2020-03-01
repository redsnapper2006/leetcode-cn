package main

import "fmt"

func uniquePaths(m int, n int) int {
	dp := make([][]int, m+1)
	for i := 0; i < m+1; i++ {
		dp[i] = make([]int, n+1)
	}
	dp[1][0] = 1
	for i := 1; i < m+1; i++ {
		for j := 1; j < n+1; j++ {
			dp[i][j] = dp[i][j-1] + dp[i-1][j]
		}
	}
	return dp[m][n]
}

func uniquePathsV2(m int, n int) int {
	if m == 1 || n == 1 {
		return 1
	}
	return uniquePaths(m-1, n) + uniquePaths(m, n-1)
}

func main() {
	fmt.Println("a")
}
