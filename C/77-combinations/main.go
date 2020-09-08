package main

import "fmt"

func combine(n int, k int) [][]int {
	buf := [][]int{}
	for i := 1; i <= n-k+1; i++ {
		buf = append(buf, []int{i})
	}

	steps := 1
	for steps < k {
		var t [][]int
		for i := 0; i < len(buf); i++ {
			b := buf[i]
			bb := b[len(b)-1]
			for j := bb + 1; j <= n-k+steps+1; j++ {
				tt := make([]int, len(b)+1)
				copy(tt, b)
				tt[len(tt)-1] = j
				t = append(t, tt)
			}
		}
		buf = t
		steps++
	}

	return buf
}

func main() {
	fmt.Println("a")
}
