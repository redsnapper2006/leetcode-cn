package main

import "fmt"

func isBipartite(graph [][]int) bool {
	M1, M2 := map[int]int{}, map[int]int{}

	for i := 0; i < len(graph); i++ {
		_, ok1 := M1[i]
		_, ok2 := M2[i]
		if ok1 || ok2 {
			continue
		}
		buf := []int{i}
		steps := 0
		for len(buf) > 0 {
			var b, l *map[int]int
			var t []int
			if steps%2 == 0 {
				b = &M1
				l = &M2
			} else {
				b = &M2
				l = &M1
			}

			for i := 0; i < len(buf); i++ {
				(*b)[buf[i]]++
			}
			for i := 0; i < len(buf); i++ {
				for j := 0; j < len(graph[buf[i]]); j++ {
					_, ok := (*b)[graph[buf[i]][j]]
					if ok {
						return false
					}
					_, ok2 := (*l)[graph[buf[i]][j]]
					if !ok2 {
						t = append(t, graph[buf[i]][j])
					}
				}
			}
			buf = t
			steps++
		}
	}

	return true
}

func main() {
	fmt.Println("a")
}
