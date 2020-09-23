package main

import "fmt"

func findSolution(customFunction func(int, int) int, z int) [][]int {
	var ret [][]int
	for i := 1; i <= 1000; i++ {
		s, e := 1, 1000
		for s <= e {
			m := s + (e-s)/2
			v := customFunction(i, m)
			if v > z {
				e = m - 1
			} else if v < z {
				s = m + 1
			} else {
				ret = append(ret, []int{i, m})
				break
			}
		}
	}
	return ret
}

func main() {
	fmt.Println("a")
}
