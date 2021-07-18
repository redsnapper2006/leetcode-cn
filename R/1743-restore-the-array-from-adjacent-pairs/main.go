package main

import "fmt"

func restoreArray(adjacentPairs [][]int) []int {
	m := map[int][]int{}
	for _, c := range adjacentPairs {
		_, ok := m[c[0]]
		if !ok {
			m[c[0]] = []int{}
		}
		m[c[0]] = append(m[c[0]], c[1])
		_, ok2 := m[c[1]]
		if !ok2 {
			m[c[1]] = []int{}
		}
		m[c[1]] = append(m[c[1]], c[0])
	}

	visited := map[int]int{}
	buf1 := []int{adjacentPairs[0][0]}
	buf2 := []int{adjacentPairs[0][0]}
	visited[adjacentPairs[0][0]] = 1

	var link func(m map[int][]int, visited map[int]int, cur int, buf *[]int)
	link = func(m map[int][]int, visited map[int]int, cur int, buf *[]int) {
		visited[cur] = 1
		*buf = append(*buf, cur)
		for _, v := range m[cur] {
			_, ok := visited[v]
			if ok {
				continue
			}
			link(m, visited, v, buf)
		}
	}

	candi := m[adjacentPairs[0][0]]
	link(m, visited, candi[0], &buf1)
	if len(candi) == 2 {
		link(m, visited, candi[1], &buf2)
	}
	fmt.Println(buf1, buf2)

	ret := []int{}
	for i := len(buf2) - 1; i >= 0; i-- {
		ret = append(ret, buf2[i])
	}
	for i := 1; i < len(buf1); i++ {
		ret = append(ret, buf1[i])
	}
	return ret
}

func main() {
	fmt.Println(restoreArray([][]int{{2, 1}, {3, 4}, {3, 2}}))
}
