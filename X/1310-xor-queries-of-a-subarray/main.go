package main

import "fmt"

func xorQueries(arr []int, queries [][]int) []int {
	buf := make([]int, len(arr))
	buf[0] = arr[0]
	for i := 1; i < len(arr); i++ {
		buf[i] = arr[i] ^ buf[i-1]
	}
	ret := []int{}
	for i := 0; i < len(queries); i++ {
		base := 0
		if queries[i][0] > 0 {
			base = buf[queries[i][0]-1]
		}
		ret = append(ret, buf[queries[i][1]]^base)
	}
	return ret
}

func main() {
	fmt.Println()
}
