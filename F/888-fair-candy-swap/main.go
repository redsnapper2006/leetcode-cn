package main

import "fmt"

func fairCandySwap(A []int, B []int) []int {
	MB := make(map[int]int)

	sum := 0
	suma := 0
	for i := 0; i < len(A); i++ {
		sum += A[i]
		suma += A[i]
	}
	for i := 0; i < len(B); i++ {
		sum += B[i]
		MB[B[i]]++
	}

	half := sum / 2
	for i := 0; i < len(A); i++ {
		_, ok := MB[half-suma+A[i]]
		if ok {
			return []int{A[i], half - suma + A[i]}
		}
	}
	return nil
}

func main() {
	fmt.Println("a")
}
