package main

import "fmt"

func networkBecomesIdle(edges [][]int, patience []int) int {
	m := map[int][]int{}
	for _, e := range edges {
		s, d := e[0], e[1]
		_, ok1 := m[s]
		if !ok1 {
			m[s] = []int{}
		}
		m[s] = append(m[s], d)
		_, ok := m[d]
		if !ok {
			m[d] = []int{}
		}
		m[d] = append(m[d], s)
	}

	visited := map[int]int{}
	visited[0] = 1
	steps := map[int]int{}
	steps[0] = 0
	candi := []int{0}
	step := 0
	for len(candi) > 0 {
		t := []int{}
		for _, c := range candi {
			steps[c] = step
			for _, v := range m[c] {
				if visited[v] == 1 {
					continue
				}
				visited[v] = 1
				t = append(t, v)
			}
		}
		candi = t
		step++
	}
	// fmt.Println(m,steps)
	max := 0
	for i := 0; i < len(patience); i++ {
		if patience[i] >= steps[i]*2 {
			if max < steps[i]*2 {
				max = steps[i] * 2
			}
			continue
		}
		times := (steps[i] * 2) / patience[i]
		if (steps[i]*2)%patience[i] == 0 {
			times--
		}
		if times*patience[i]+steps[i]*2 > max {
			max = times*patience[i] + steps[i]*2
		}
	}
	return max + 1
}

func main() {
	fmt.Println()
}
