package main

import (
	"fmt"
)

func superEggDrop(K int, N int) int {
	// F(K,N) = min(1..n, max(F(K-1,M), F(K,N-M)))
	if K == 1 {
		return N
	}
	if N <= 2 {
		return N
	}
	buf := make([][]int, N+1)
	for i := 0; i < len(buf); i++ {
		buf[i] = make([]int, K+1)
	}

	for i := 1; i <= K; i++ {
		buf[1][i] = 1
	}
	for i := 1; i <= N; i++ {
		buf[i][1] = i
	}

	for j := 2; j <= N; j++ {
		for i := 2; i <= K; i++ {
			s := buf[j-1][i-1] + buf[j-1][i] + 1
			buf[j][i] = s
			if s >= N {
				return j
			}
		}
	}
	return 0
}

func superEggDropV2(K int, N int) int {
	// F(K,N) = min(1..n, max(F(K-1,M), F(K,N-M)))

	buf := make([][]int, N+1)
	for i := 0; i < len(buf); i++ {
		buf[i] = make([]int, K+1)
	}

	for i := 1; i <= K; i++ {
		buf[1][i] = 1
	}
	for i := 1; i <= N; i++ {
		buf[i][1] = i
	}
	if K == 1 {
		return buf[N][1]
	}
	if N == 1 {
		return buf[1][K]
	}

	for j := 2; j <= N; j++ {
		for i := 2; i <= K; i++ {
			c := N + 1
			for n := 1; n <= j; n++ {
				t := buf[n-1][i-1]
				if buf[n-1][i-1] < buf[j-n][i] {
					t = buf[j-n][i]
				}
				if c > t+1 {
					c = t + 1
				}
			}
			buf[j][i] = c
		}
	}
	return buf[N][K]
}

func main() {
	fmt.Println(superEggDrop(2, 2))
	fmt.Println(superEggDrop(2, 6))
	fmt.Println(superEggDrop(3, 14))
	fmt.Println(superEggDrop(100, 10000))
}
