package main

import "fmt"

func canCross(stones []int) bool {
	MAX := stones[len(stones)-1]
	M := map[int]int{}
	for _, v := range stones {
		M[v]++
	}
	MH := map[int]map[int]bool{}

	var dfs func(pos int, step int, M map[int]int, MH map[int]map[int]bool, MAX int) bool
	dfs = func(pos int, step int, M map[int]int, MH map[int]map[int]bool, MAX int) bool {
		if pos == MAX {
			return true
		}
		k1, k2, k3 := false, false, false
		if M[pos+step+1] > 0 {
			_, ok := MH[pos]
			if !ok {
				MH[pos] = map[int]bool{}
			}
			v, ok2 := MH[pos][step+1]
			if !ok2 {
				k1 = dfs(pos+step+1, step+1, M, MH, MAX)
				MH[pos][step+1] = k1
				if k1 {
					return k1
				}
			} else if v {
				return v
			}
		}

		if step > 0 && M[pos+step] > 0 {
			_, ok := MH[pos]
			if !ok {
				MH[pos] = map[int]bool{}
			}
			v, ok2 := MH[pos][step]
			if !ok2 {
				k2 = dfs(pos+step, step, M, MH, MAX)
				MH[pos][step] = k2
				if k2 {
					return k2
				}
			} else if v {
				return v
			}
		}

		if step-1 > 0 && M[pos+step-1] > 0 {
			_, ok := MH[pos]
			if !ok {
				MH[pos] = map[int]bool{}
			}
			v, ok2 := MH[pos][step-1]
			if !ok2 {
				k3 = dfs(pos+step-1, step-1, M, MH, MAX)
				MH[pos][step-1] = k3
				return k3
			} else if v {
				return v
			}
		}
		return false
	}
	return dfs(0, 0, M, MH, MAX)
}

func main() {
	fmt.Println(canCross([]int{0, 1, 3, 5, 6, 8, 12, 17}))
}
