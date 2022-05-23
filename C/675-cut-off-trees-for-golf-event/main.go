package main

import "sort"

func cutOffTree(forest [][]int) int {
	pos := map[int][]int{}
	min, max := 1000000001, -1
	for i := 0; i < len(forest); i++ {
		for j := 0; j < len(forest[0]); j++ {
			if forest[i][j] == 0 || forest[i][j] == 1 {
				continue
			}
			pos[forest[i][j]] = []int{i, j}
			if forest[i][j] < min {
				min = forest[i][j]
			}
			if forest[i][j] > max {
				max = forest[i][j]
			}
		}
	}
	keys := []int{}
	for k := range pos {
		keys = append(keys, k)
	}
	sort.Ints(keys)

	cords := [][]int{{-1, 0}, {1, 0}, {0, -1}, {0, 1}}
	steps := 0
	r, c := 0, 0
	for _, key := range keys {
		target, ok := pos[key]
		if !ok {
			continue
		}

		candi := [][]int{{r, c}}
		visit := make([][]int, len(forest))
		for i := 0; i < len(visit); i++ {
			visit[i] = make([]int, len(forest[0]))
		}
		visit[r][c] = 1
		isFound := false
		for len(candi) > 0 {
			t := [][]int{}
			for _, v := range candi {
				if v[0] == target[0] && v[1] == target[1] {
					isFound = true
					break
				}

				for _, cord := range cords {
					if v[0]+cord[0] < 0 || v[0]+cord[0] >= len(forest) || v[1]+cord[1] < 0 || v[1]+cord[1] >= len(forest[0]) || visit[v[0]+cord[0]][v[1]+cord[1]] == 1 || forest[v[0]+cord[0]][v[1]+cord[1]] == 0 {
						continue
					}
					visit[v[0]+cord[0]][v[1]+cord[1]] = 1
					t = append(t, []int{v[0] + cord[0], v[1] + cord[1]})
				}
			}
			if isFound {
				break
			}
			steps++
			candi = t
		}
		if !isFound {
			return -1
		}
		r, c = target[0], target[1]
	}
	return steps
}
