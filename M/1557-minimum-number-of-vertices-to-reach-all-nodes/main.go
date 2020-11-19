package main

import "fmt"

func findSmallestSetOfVertices(n int, edges [][]int) []int {
	buf := make([]int, n)
	for _, v := range edges {
		buf[v[1]]++
	}
	var ret []int
	for i := 0; i < n; i++ {
		if buf[i] == 0 {
			ret = append(ret, i)
		}
	}
	return ret
}

func main() {
	fmt.Println()
}
