package main

import "fmt"

func findWhetherExistsPath(n int, graph [][]int, start int, target int) bool {
	V := map[int]int{}
	E := map[int]map[int]int{}

	for i := 0; i < len(graph); i++ {
		conn := graph[i]
		if conn[0] == conn[1] {
			continue
		}
		_, ok := E[conn[0]]
		if !ok {
			E[conn[0]] = map[int]int{}
		}
		E[conn[0]][conn[1]]++
	}

	buf := []int{start}
	for len(buf) > 0 {
		var bb []int
		for i := 0; i < len(buf); i++ {
			candi := E[buf[i]]
			for k := range candi {
				if k == target {
					return true
				}
				_, ok := V[k]
				if !ok {
					bb = append(bb, k)
					V[k]++
				}
			}
		}
		if len(bb) == 0 {
			break
		}
		buf = bb
	}
	return false
}

func main() {
	fmt.Println("a")
}
