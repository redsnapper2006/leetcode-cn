package main

func findLUSlength(strs []string) int {
	ans := -1
	for i := 0; i < len(strs); i++ {
		isSub := false
		for j := 0; j < len(strs); j++ {
			if i == j {
				continue
			}

			i1, j1 := 0, 0
			for i1 < len(strs[i]) && j1 < len(strs[j]) {
				if strs[i][i1] == strs[j][j1] {
					i1++
				}
				j1++
			}
			isSub = isSub || i1 == len(strs[i])
		}

		if !isSub && len(strs[i]) > ans {
			ans = len(strs[i])
		}
	}
	return ans
}
