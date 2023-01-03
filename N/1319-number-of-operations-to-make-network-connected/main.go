package main

import "fmt"

func makeConnected(n int, connections [][]int) int {
	M := make([][]int, n)
	cnt := 0
	for _, c := range connections {
		cnt++
		if len(M[c[0]]) == 0 {
			M[c[0]] = []int{}
		}
		M[c[0]] = append(M[c[0]], c[1])
		if len(M[c[1]]) == 0 {
			M[c[1]] = []int{}
		}
		M[c[1]] = append(M[c[1]], c[0])
	}
	if cnt < n-1 {
		return -1
	}
	R := make([]int, n)
	for i := 0; i < len(M); i++ {
		R[i] = -1
	}
	for i := 0; i < len(M); i++ {
		if R[i] == -1 {
			R[i] = i
		}
		parent := i
		t := M[i]
		for len(t) > 0 {
			var temp []int
			for _, b := range t {
				if R[b] == -1 {
					R[b] = parent
					temp = append(temp, M[b]...)
				}
			}
			t = temp
		}
	}

	RR := map[int]int{}
	c2 := 0
	for i := 0; i < n; i++ {
		if R[i] == -1 {
			c2++
		} else {
			RR[R[i]]++
		}
	}
	return len(RR) + c2 - 1
}

func main() {
	fmt.Println()
}
