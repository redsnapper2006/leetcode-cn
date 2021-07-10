package main

import "fmt"

func numEquivDominoPairs(dominoes [][]int) int {
	M := map[int]int{}
	for _, domino := range dominoes {
		sum := 0
		for _, c := range domino {
			sum = sum*10 + c
		}
		rsum := 0
		for i := len(domino) - 1; i >= 0; i-- {
			rsum = rsum*10 + domino[i]
		}
		M[sum]++
		M[rsum]++
	}

	ret := 0
	for k, v := range M {
		if v == 1 {
			continue
		}
		var b []int
		t := k
		for t > 0 {
			b = append(b, t%10)
			t /= 10
		}
		s, e := 0, len(b)-1
		isSymmetry := true
		for s < e {
			if b[s] != b[e] {
				isSymmetry = false
				break
			}
			s++
			e--
		}
		if isSymmetry {
			v /= 2
			ret += v * (v - 1)
		} else {
			ret += v * (v - 1) / 2
		}
	}
	return ret / 2
}

func main() {
	fmt.Println("a")
}
