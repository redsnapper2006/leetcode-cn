package main

import "fmt"

func waysToChange(n int) int {
	buf := make([][]int, 4)
	for i := 0; i < 4; i++ {
		buf[i] = make([]int, n+1)
	}
	buf[0][0] = 1
	buf[1][0] = 0
	buf[2][0] = 0
	buf[3][0] = 0

	for i := 0; i <= n; i++ {
		if i+1 <= n {
			buf[0][i+1] += buf[0][i]
		}
		if i+5 <= n {
			buf[1][i+5] += buf[0][i]
			buf[1][i+5] += buf[1][i]
		}
		if i+10 <= n {
			buf[2][i+10] += buf[0][i]
			buf[2][i+10] += buf[1][i]
			buf[2][i+10] += buf[2][i]
		}
		if i+25 <= n {
			buf[3][i+25] += buf[0][i]
			buf[3][i+25] += buf[1][i]
			buf[3][i+25] += buf[2][i]
			buf[3][i+25] += buf[3][i]
		}
	}
	// fmt.Println(buf)
	return (buf[0][n] + buf[1][n] + buf[2][n] + buf[3][n]) % 1000000007
}

func waysToChangeV2(n int) int {

	var recur func(n int, max int) int
	recur = func(n int, max int) int {
		if n == 0 {
			return 1
		}
		s := 0
		if n >= 25 && max >= 25 {
			s += recur(n-25, 25)
		}
		if n >= 10 && max >= 10 {
			s += recur(n-10, 10)
		}
		if n >= 5 && max >= 5 {
			s += recur(n-5, 5)
		}
		if n >= 1 && max >= 1 {
			s += recur(n-1, 1)
		}
		return s
	}
	return recur(n, 25)
}

func main() {
	fmt.Println(waysToChange(25))
}
