package main

import "fmt"

func findMinHeightTrees(n int, edges [][]int) []int {
	if n == 1 {
		return []int{0}
	}

	link := make([][]int, n)
	for _, e := range edges {
		x, y := e[0], e[1]
		link[x] = append(link[x], y)
		link[y] = append(link[y], x)
	}
	parents := make([]int, n)
	bfs := func(start int) int {
		vis := make([]bool, n)
		vis[start] = true
		q := []int{start}
		x := -1
		for len(q) > 0 {
			x, q = q[0], q[1:]
			for _, y := range link[x] {
				if !vis[y] {
					vis[y] = true
					parents[y] = x
					q = append(q, y)
				}
			}
		}
		return x
	}

	x := bfs(0)
	y := bfs(x)

	path := []int{}
	parents[x] = -1
	for y != -1 {
		path = append(path, y)
		y = parents[y]
	}

	m := len(path)
	if m%2 == 0 {
		return []int{path[m/2-1], path[m/2]}
	}
	return []int{path[m/2]}
}

func findMinHeightTrees2(n int, edges [][]int) []int {
	nodes := make([]int, n)
	for i := 0; i < n; i++ {
		nodes[i] = i
	}
	link := map[int]map[int]int{}
	for i := 0; i < len(edges); i++ {
		s, e := edges[i][0], edges[i][1]
		_, ok := link[s]
		if !ok {
			link[s] = map[int]int{}
		}
		link[s][e]++
		_, ok2 := link[e]
		if !ok2 {
			link[e] = map[int]int{}
		}
		link[e][s]++
	}

	for len(link) > 2 {
		oneGress := []int{}
		refresh := [][]int{}
		for k, v := range link {
			if len(v) == 1 {
				oneGress = append(oneGress, k)
				for k1 := range v {
					refresh = append(refresh, []int{k1, k})
				}
			}
		}
		for i := 0; i < len(oneGress); i++ {
			delete(link, oneGress[i])
		}
		for i := 0; i < len(refresh); i++ {
			delete(link[refresh[i][0]], refresh[i][1])
		}
	}
	if len(link) > 0 {
		var ret []int
		for k := range link {
			ret = append(ret, k)
		}
		return ret
	}

	return nodes
}

func main() {
	fmt.Println(findMinHeightTrees(9, [][]int{{0, 1}, {0, 2}, {2, 3}, {0, 4}, {2, 5}, {5, 6}, {3, 7}, {0, 8}}))
}
