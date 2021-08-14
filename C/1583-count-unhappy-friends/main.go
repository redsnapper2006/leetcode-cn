package main

import "fmt"

func unhappyFriends(n int, preferences [][]int, pairs [][]int) int {
	preferMap := map[int][]int{}
	for i, prefer := range preferences {
		preferMap[i] = make([]int, n)
		for j, p := range prefer {
			preferMap[i][p] = j + 1
		}
	}
	pairMap := map[int]int{}
	for _, p := range pairs {
		pairMap[p[0]] = p[1]
		pairMap[p[1]] = p[0]
	}

	ret := 0
	for i := 0; i < n; i++ {
		x := i
		y := pairMap[x]
		idx := preferMap[x][y]
		for j := 0; j < idx; j++ {
			u := preferences[x][j]
			v := pairMap[u]
			if preferMap[u][x] < preferMap[u][v] {
				ret++
				break
			}
		}
	}
	return ret
}

func main() {
	fmt.Println()
}
