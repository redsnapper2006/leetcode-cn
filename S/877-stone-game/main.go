package main

import "fmt"

func stoneGame(piles []int) bool {
	buf := make([][]int, len(piles))
	for i := 0; i < len(piles); i++ {
		buf[i] = make([]int, len(piles))
		buf[i][i] = piles[i]
	}
	for i := len(piles) - 2; i >= 0; i-- {
		for j := i + 1; j < len(piles); j++ {
			c := piles[i] - buf[i+1][j]
			if c < piles[j]-buf[i][j-1] {
				c = piles[j] - buf[i][j-1]
			}
			buf[i][j] = c
		}
	}
	return buf[0][len(piles)-1] > 0
}

func stoneGameV2(piles []int) bool {
	M := map[string]bool{}
	var recur func(piles []int, A int, L int, AorL bool, S int, E int) bool
	recur = func(piles []int, A int, L int, AorL bool, S int, E int) bool {
		// fmt.Println("begin", piles, A, L, AorL)
		if S > E {
			return A > L
		}
		k := fmt.Sprintf("%d_%d", S, E)
		v, ok := M[k]
		if ok {
			return v
		}

		if S == E {

			a, l := 0, 0
			if AorL {
				a = piles[S]
			} else {
				l = piles[E]
			}
			r := recur([]int{}, A+a, L+l, !AorL, S+1, E)
			M[k] = r
			return r
		}

		a, l := 0, 0
		if AorL {
			a = piles[S]
		} else {
			l = piles[S]
		}
		r1 := recur(piles, A+a, L+l, !AorL, S+1, E)
		a, l = 0, 0
		if AorL {
			a = piles[E]
		} else {
			l = piles[E]
		}
		r2 := recur(piles, A+a, L+l, !AorL, S, E-1)
		M[k] = r1 || r2
		// fmt.Println("result", r1, r2)
		return r1 || r2
	}

	return recur(piles, 0, 0, true, 0, len(piles)-1)
}

func main() {
	fmt.Println(stoneGame([]int{5, 3, 4, 5}))
}
