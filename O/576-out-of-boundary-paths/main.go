package main

import "fmt"

func findPaths(m int, n int, N int, i int, j int) int {
	if N == 0 {
		return 0
	}
	p0 := make([][]int, m)
	for i := 0; i < m; i++ {
		p0[i] = make([]int, n)
	}
	p1 := make([][]int, m)
	for i := 0; i < m; i++ {
		p1[i] = make([]int, n)
	}
	for i := 0; i < m; i++ {
		for j := 0; j < n; j++ {
			r := 0
			if i == 0 {
				r++
			}
			if j == 0 {
				r++
			}
			if i == m-1 {
				r++
			}
			if j == n-1 {
				r++
			}
			p1[i][j] = r
		}
	}

	sum := p1[i][j]
	for s := 2; s <= N; s++ {
		var w, r *[][]int
		if s%2 == 0 {
			w = &p0
			r = &p1
		} else {
			w = &p1
			r = &p0
		}
		for i := 0; i < m; i++ {
			for j := 0; j < n; j++ {
				a := 0
				if i > 0 {
					a += (*r)[i-1][j] % 1000000007
				}
				if j > 0 {
					a += (*r)[i][j-1] % 1000000007
				}
				if i < m-1 {
					a += (*r)[i+1][j] % 1000000007
				}
				if j < n-1 {
					a += (*r)[i][j+1] % 1000000007
				}
				(*w)[i][j] = (a % 1000000007)
			}
		}
		sum += (*w)[i][j]
		sum %= 1000000007
	}

	return sum
}

func main() {
	fmt.Println("a")
}
