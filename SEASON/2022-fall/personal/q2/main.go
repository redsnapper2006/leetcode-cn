package main

func transportationHub(path [][]int) int {
	point := map[int]int{}
	edge := map[int]map[int]int{}
	reverse := map[int][]int{}
	for _, p := range path {
		point[p[0]]++
		point[p[1]]++
		_, ok := edge[p[0]]
		if !ok {
			edge[p[0]] = map[int]int{}
		}
		edge[p[0]][p[1]]++
		_, ok2 := reverse[p[0]]
		if !ok2 {
			reverse[p[0]] = []int{}
		}
		reverse[p[1]] = append(reverse[p[1]], p[0])
	}

	for k := range point {
		_, ok := edge[k]
		if ok {
			continue
		}
		v, ok := reverse[k]
		if !ok {
			continue
		}
		visited := map[int]int{}
		candi := v
		for len(candi) > 0 {
			t := []int{}
			for _, c := range candi {
				_, ok := visited[c]
				if ok {
					continue
				}
				t = append(t, c)
				visited[c] = 1
			}
			candi = t
		}
		if len(visited) == len(point)-1 {
			return k
		}
	}

	return -1
}
