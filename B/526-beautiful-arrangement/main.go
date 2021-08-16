package main

import "fmt"

func countArrangement(N int) int {
	cnt := 0

	visit := map[int]int{}

	var dfs func(idx int, visit map[int]int, match map[int][]int)
	dfs = func(idx int, visit map[int]int, match map[int][]int) {
		if idx == N && len(visit) == N {
			for _, v := range visit {
				if v == 0 {
					return
				}
			}
			cnt++
			return
		}
		candi := match[idx+1]
		for _, c := range candi {
			v := visit[c]
			// fmt.Println(visit, c, idx, match)
			if v == 1 {
				continue
			}
			visit[c] = 1
			dfs(idx+1, visit, match)
			visit[c] = 0
		}
	}

	match := map[int][]int{}
	for i := 1; i <= N; i++ {
		match[i] = []int{}
		for j := 1; j <= N; j++ {
			if i%j == 0 || j%i == 0 {
				match[i] = append(match[i], j)
			}
		}
	}

	dfs(0, visit, match)
	return cnt
}

func main() {
	fmt.Println(countArrangement(13))
}
