package main

func validPath(n int, edges [][]int, source int, destination int) bool {
	edgeMap := map[int][]int{}
	for _, edge := range edges {
		e1, e2 := edge[0], edge[1]
		_, ok := edgeMap[e1]
		if !ok {
			edgeMap[e1] = []int{}
		}
		edgeMap[e1] = append(edgeMap[e1], e2)
		_, ok2 := edgeMap[e2]
		if !ok2 {
			edgeMap[e2] = []int{}
		}
		edgeMap[e2] = append(edgeMap[e2], e1)
	}

	visited := map[int]int{}
	candis := []int{source}
	for len(candis) > 0 {
		t := []int{}
		for _, candi := range candis {
			if candi == destination {
				return true
			}
			visited[candi] = 1
			next := edgeMap[candi]
			for _, n := range next {
				_, ok := visited[n]
				if ok {
					continue
				}
				t = append(t, n)
			}
		}
		candis = t
	}
	return false
}
