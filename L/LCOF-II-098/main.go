package main

func uniquePaths(m int, n int) int {
	buf := make([][]int, m)
	for i := 0; i < m; i++ {
		buf[i] = make([]int, n)
		for j := 0; j < n; j++ {
			if i == 0 && j == 0 {
				buf[i][j] = 1
				continue
			}
			up, left := 0, 0
			if i > 0 {
				up = buf[i-1][j]
			}
			if j > 0 {
				left = buf[i][j-1]
			}

			buf[i][j] = up + left
		}
	}
	return buf[m-1][n-1]
}
