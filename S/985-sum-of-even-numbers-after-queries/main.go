package main

import "fmt"

func sumEvenAfterQueries(A []int, queries [][]int) []int {
	sum := 0

	for _, v := range A {
		if v%2 == 0 {
			sum += v
		}
	}
	var ret []int
	for _, c := range queries {
		v, idx := c[0], c[1]
		if A[idx]%2 == 0 {
			sum -= A[idx]
		}
		A[idx] += v
		if A[idx]%2 == 0 {
			sum += A[idx]
		}
		ret = append(ret, sum)
	}
	return ret
}

func main() {
	fmt.Println("a")
}
