package main

import "fmt"

func countVowelStrings(n int) int {
	M := map[int]map[int]int{}

	var recur func(n, s int) int
	recur = func(n, s int) int {
		if n == 1 {
			return 5 - s
		}

		sum := 0
		for i := s; i < 5; i++ {
			v, ok := M[i]
			isTrain := false
			if !ok {
				isTrain = true
				M[i] = map[int]int{}
			} else {
				_, ok2 := v[n]
				if !ok2 {
					isTrain = true
				}
			}

			if isTrain {
				r := recur(n-1, i)
				M[i][n] = r
			}
			sum += M[i][n]

		}
		return sum
	}
	return recur(n, 0)
}

func main() {
	fmt.Println()
}
