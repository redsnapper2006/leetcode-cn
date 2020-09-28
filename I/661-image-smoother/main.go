package main

import "fmt"

func imageSmoother(M [][]int) [][]int {
	ret := make([][]int, len(M))
	sum := make([][]int, len(M))
	for i, v := range M {
		ret[i] = make([]int, len(M[0]))
		sum[i] = make([]int, len(M[0]))
		s := 0
		for j, c := range v {
			s += c
			sum[i][j] = s
		}
	}
	calc := func(sum [][]int, i, j int) (int, int) {
		s, e := 0, 0
		c := 0
		c++
		if j > 0 {
			c++
		}
		if j < len(sum[0])-1 {
			c++
		}
		if j > 1 {
			s = sum[i][j-2]
		}
		if j == len(sum[0])-1 {
			e = sum[i][j]
		} else {
			e = sum[i][j+1]
		}

		return e - s, c
	}

	for i, v := range sum {
		for j := range v {
			prev, pcnt := 0, 0
			if i > 0 {
				prev, pcnt = calc(sum, i-1, j)
			}
			later, lcnt := 0, 0
			if i < len(sum)-1 {
				later, lcnt = calc(sum, i+1, j)
			}
			cur, ccnt := calc(sum, i, j)
			ret[i][j] = (prev + later + cur) / (pcnt + lcnt + ccnt)
		}
	}
	return ret
}

func main() {
	fmt.Println("a")
}
