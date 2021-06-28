package main

import (
	"fmt"
)

func numBusesToDestination(routes [][]int, source int, target int) int {
	if source == target {
		return 0
	}
	idxMap := map[int][]int{}
	for i := 0; i < len(routes); i++ {
		r := routes[i]
		for j := 0; j < len(r); j++ {
			_, ok := idxMap[r[j]]
			if !ok {
				idxMap[r[j]] = []int{}
			}
			idxMap[r[j]] = append(idxMap[r[j]], i)
		}
	}

	visited := map[int]int{}
	working := []int{target}
	steps := 0
	for len(working) > 0 {
		// fmt.Println(working)
		next := []int{}
		// mm := map[int]int{}
		for i := 0; i < len(working); i++ {
			for j := 0; j < len(idxMap[working[i]]); j++ {
				_, ok := visited[idxMap[working[i]][j]]
				if !ok {
					visited[idxMap[working[i]][j]] = 1
					route := routes[idxMap[working[i]][j]]
					for m := 0; m < len(route); m++ {
						if route[m] == source {
							return steps + 1
						}
						next = append(next, route[m])
					}
				}
			}
		}

		steps++
		working = next
	}
	return -1
}

func main() {
	fmt.Println(numBusesToDestination([][]int{{16, 25, 29, 35, 42}, {32}, {1}, {1, 2, 5, 17, 22, 34, 38, 41, 44}, {1, 16, 23, 24, 32, 36, 38, 45}, {9, 11, 43}, {5, 10, 15, 22, 27, 38}, {7, 8, 14, 19, 22, 23, 25, 26, 27}, {3, 8, 24, 29, 47, 48}, {4, 16, 18, 25, 36, 41}}, 17, 38))
}
