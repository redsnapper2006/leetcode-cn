package main

import "fmt"

func combinationSum3(k int, n int) [][]int {
	var recur func(k, n, s int) [][]int
	recur = func(k, n, s int) [][]int {
		if k == 1 {
			if s <= n && s < 10 && n < 10 {
				return [][]int{{n}}
			}
			return nil
		}
		var ret [][]int
		for i := s; i < 10; i++ {
			if i > n {
				continue
			}
			tr := recur(k-1, n-i, i+1)
			for m := 0; m < len(tr); m++ {
				t := make([]int, len(tr[m])+1)
				copy(t[1:], tr[m])
				t[0] = i
				ret = append(ret, t)
			}
		}
		return ret
	}
	return recur(k, n, 1)
}

func main() {
	fmt.Println("a")
}
