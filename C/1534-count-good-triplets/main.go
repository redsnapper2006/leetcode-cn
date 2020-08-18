package main

import "fmt"

func countGoodTriplets(arr []int, a int, b int, c int) int {
	target := 0
	for i := 0; i < len(arr); i++ {
		for j := i + 1; j < len(arr); j++ {
			ij := arr[i] - arr[j]
			if ij < 0 {
				ij = -ij
			}
			for l := j + 1; l < len(arr); l++ {
				jl := arr[j] - arr[l]
				if jl < 0 {
					jl = -jl
				}
				il := arr[i] - arr[l]
				if il < 0 {
					il = -il
				}
				if ij <= a && jl <= b && il <= c {
					target++
				}
			}
		}
	}
	return target
}

func main() {
	fmt.Println("a")
}
