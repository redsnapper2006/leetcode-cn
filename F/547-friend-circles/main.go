package main

import "fmt"

func findCircleNum(M [][]int) int {

	c := 0
	for i := 0; i < len(M); i++ {
		var b []int
		for j := 0; j < len(M[0]); j++ {
			if M[i][j] == 1 {
				M[i][j] = 0
				b = append(b, j)
			}
		}
		if len(b) == 0 {
			continue
		}
		for len(b) > 0 {
			idx := b[0]
			for m := 0; m < len(M[0]); m++ {
				if M[idx][m] == 1 {
					M[idx][m] = 0
					b = append(b, m)
				}
			}
			b = b[1:]
		}
		c++
	}
	return c
}

func main() {
	fmt.Println("a")
}
