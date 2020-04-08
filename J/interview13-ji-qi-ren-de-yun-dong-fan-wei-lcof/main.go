package main

import "fmt"

func movingCount(m int, n int, k int) int {
	M := make(map[int]int)
	for i := 0; i < 100; i++ {
		d1 := i / 10
		d2 := i % 10
		M[i] = d1 + d2
	}
	M[100] = 1

	buf := make([][]int, m)
	for i := 0; i < m; i++ {
		buf[i] = make([]int, n)
	}

	for i := 0; i < m; i++ {
		b1 := M[i]
		if b1 > k {
			break
		}
		for j := 0; j < n; j++ {
			b2 := M[j]
			if b1+b2 <= k && ((i == 0 && j == 0) || (i > 0 && buf[i-1][j] == 1) || (j > 0 && buf[i][j-1] == 1)) {
				buf[i][j] = 1
			}
		}
	}
	c := 0
	for i := 0; i < m; i++ {
		for j := 0; j < n; j++ {
			c += buf[i][j]
		}
	}

	return c
}

func main() {
	fmt.Println(movingCount(16, 8, 4))
	fmt.Println(movingCount(38, 15, 9))
}
