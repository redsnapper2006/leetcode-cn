package main

func possibleBipartition(n int, dislikes [][]int) bool {
	edge := map[int][]int{}
	for i := 0; i < len(dislikes); i++ {
		dl := dislikes[i]
		d1, d2 := dl[0], dl[1]
		_, ok := edge[d1]
		if !ok {
			edge[d1] = []int{}
		}
		edge[d1] = append(edge[d1], d2)
		_, ok2 := edge[d2]
		if !ok2 {
			edge[d2] = []int{}
		}
		edge[d2] = append(edge[d2], d1)
	}

	visited := map[int]int{}
	for i := 1; i <= n; i++ {
		_, ok := visited[i]
		if ok {
			continue
		}
		candi := []int{i}
		color := 0

		for len(candi) > 0 {
			t := []int{}
			for m := 0; m < len(candi); m++ {
				c := candi[m]
				visited[c] = color
				next := edge[c]
				for _, n := range next {
					ncolor, ok2 := visited[n]
					if ok2 {
						if ncolor == color {
							return false
						}
						continue
					}
					t = append(t, n)
				}
			}
			candi = t
			color++
			color %= 2
		}
	}
	return true
}
