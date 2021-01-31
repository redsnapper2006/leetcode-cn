package main

import (
	"fmt"
	"sort"
)

func numSimilarGroups(strs []string) int {
	M := map[int][]int{}
	for i := 0; i < len(strs); i++ {
		target := strs[i]
		for j := i + 1; j < len(strs); j++ {
			candi := strs[j]
			isFirst := true
			isSecond := true
			isTarget := false
			diff1, diff2 := byte('0'), byte('0')
			for n := 0; n < len(target); n++ {
				if target[n] != candi[n] {
					if isFirst {
						isFirst = false
						diff1 = target[n]
						diff2 = candi[n]
					} else if isSecond && diff1 == candi[n] && diff2 == target[n] {
						isSecond = false
						isTarget = true
					} else {
						isTarget = false
						break
					}
				}
			}
			if isTarget || isFirst {
				_, ok1 := M[i]
				if !ok1 {
					M[i] = []int{}
				}
				M[i] = append(M[i], j)
				_, ok2 := M[j]
				if !ok2 {
					M[j] = []int{}
				}
				M[j] = append(M[j], i)
			}
		}
	}
	var dfs func(buf []int, M map[int][]int, index, parent int)
	dfs = func(buf []int, M map[int][]int, index, parent int) {
		for i := 0; i < len(M[index]); i++ {
			if buf[M[index][i]] != -1 {
				continue
			}
			buf[M[index][i]] = parent
			dfs(buf, M, M[index][i], parent)
		}
	}
	buf := make([]int, len(strs))
	for i := 0; i < len(strs); i++ {
		buf[i] = -1
	}
	for i := 0; i < len(strs); i++ {
		if buf[i] != -1 {
			continue
		}
		buf[i] = i
		dfs(buf, M, i, i)
	}
	sort.Ints(buf)
	cnt := 1
	base := buf[0]
	for i := 1; i < len(buf); i++ {
		if buf[i] == base {
			continue
		}
		cnt++
		base = buf[i]
	}
	return cnt
}

func main() {
	fmt.Println()
}
