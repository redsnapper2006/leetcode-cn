package main

import (
	"fmt"
	"sort"
)

func eventualSafeNodes(graph [][]int) []int {
	IN := map[int][]int{}
	OUT := map[int][]int{}

	for i := 0; i < len(graph); i++ {
		OUT[i] = graph[i]
		for j := 0; j < len(graph[i]); j++ {
			IN[graph[i][j]] = append(IN[graph[i][j]], i)
		}
	}

	var ret []int
	var w []int
	for i := 0; i < len(graph); i++ {
		if len(OUT[i]) == 0 {
			ret = append(ret, i)
			w = append(w, i)
		}
	}
	for len(w) > 0 {
		buf := map[int]int{}
		for i := 0; i < len(w); i++ {
			v := IN[w[i]]
			for j := 0; j < len(v); j++ {
				buf[v[j]]++
				a := OUT[v[j]]
				idx := -1
				for m := 0; m < len(a); m++ {
					if a[m] == w[i] {
						idx = m
						break
					}
				}
				OUT[v[j]] = append(a[0:idx], a[idx+1:]...)
			}
		}
		var t []int
		for k, _ := range buf {
			if len(OUT[k]) == 0 {
				t = append(t, k)
				ret = append(ret, k)
			}
		}
		w = t
	}
	sort.Ints(ret)
	return ret
}

func main() {
	fmt.Println("a")
}
