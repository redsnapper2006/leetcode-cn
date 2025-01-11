package main

import "fmt"

func numWays(n int, relation [][]int, k int) int {
	M := map[int][]int{}
	for i := 0; i < len(relation); i++ {
		_, ok := M[relation[i][0]]
		if !ok {
			M[relation[i][0]] = []int{}
		}
		M[relation[i][0]] = append(M[relation[i][0]], relation[i][1])
	}

	stack := []int{0}
	m := map[int]int{0: 1}
	steps := map[int]int{}
	for k > 0 && len(stack) > 0 {
		var t []int
		steps = map[int]int{}
		for i := 0; i < len(stack); i++ {
			candi := M[stack[i]]
			for j := 0; j < len(candi); j++ {
				steps[candi[j]] += m[stack[i]]
			}
		}
		for k := range steps {
			t = append(t, k)
		}
		stack = t
		m = steps
		k--
	}

	return m[n-1]
}

func main() {
	fmt.Println("a")
}
