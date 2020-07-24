package main

import "fmt"

func divisorGame(N int) bool {
	buf := make([]bool, N+1)
	buf[1] = false
	for i := 2; i <= N; i++ {
		isOK := false
		for j := 1; j <= i/2; j++ {
			if i%j == 0 && buf[i-j] == false {
				isOK = true
				break
			}
		}
		buf[i] = isOK
	}
	return buf[N]
}

func divisorGameV2(N int) bool {
	return N%2 == 0
}

func main() {
	fmt.Println("a")
}
