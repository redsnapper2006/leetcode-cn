package main

import "fmt"

func findMinHeightTrees(n int, edges [][]int) []int {
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

	for len(nodes) > 2 {
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
		var n []int
		for i := 0; i < len(nodes); i++ {
			isFound := false
			for j := 0; j < len(oneGress); j++ {
				if nodes[i] == oneGress[j] {
					isFound = true
					break
				}
			}
			if !isFound {
				n = append(n, nodes[i])
			}
		}
		nodes = n
	}

	return nodes
}

func main() {
	fmt.Println(findMinHeightTrees(9, [][]int{{0, 1}, {0, 2}, {2, 3}, {0, 4}, {2, 5}, {5, 6}, {3, 7}, {0, 8}}))
}
