package main

import "fmt"

func countTriplets(arr []int) int {
	M := map[int][]int{}
	M[0] = []int{-1}

	ret := 0
	base := 0
	for i := 0; i < len(arr); i++ {
		base ^= arr[i]
		// fmt.Println(base, M)
		v, ok := M[base]
		if !ok {
			M[base] = []int{}
		} else {
			for j := 0; j < len(v); j++ {
				ret += i - v[j] - 1
			}
		}
		M[base] = append(M[base], i)
	}
	return ret
}

func main() {
	fmt.Println(countTriplets([]int{2, 3, 1, 6, 7}))
	fmt.Println(countTriplets([]int{1, 1, 1, 1, 1}))
	fmt.Println(countTriplets([]int{7, 11, 12, 9, 5, 2, 7, 17, 22}))
}
