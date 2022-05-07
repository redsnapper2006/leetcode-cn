package main

func minMutation(start string, end string, bank []string) int {
	m := map[string][]string{}

	for i := 0; i < len(bank); i++ {
		for j := i + 1; j < len(bank); j++ {
			cnt := 0
			for n := 0; n < 8; n++ {
				if bank[i][n] != bank[j][n] {
					cnt++
				}
			}
			if cnt == 1 {
				m[bank[i]] = append(m[bank[i]], bank[j])
				m[bank[j]] = append(m[bank[j]], bank[i])
			}
		}
	}

	for i := 0; i < len(bank); i++ {
		cnt := 0
		for n := 0; n < 8; n++ {
			if bank[i][n] != start[n] {
				cnt++
			}
		}
		if cnt == 1 {
			m[start] = append(m[start], bank[i])
		}
	}

	visit := map[string]int{}
	visit[start] = 1

	max := -1
	var dfs func(m map[string][]string, visit map[string]int, base string, steps int)
	dfs = func(m map[string][]string, visit map[string]int, base string, steps int) {
		if base == end {
			if max == -1 || max > steps {
				max = steps
			}
			return
		}
		visit[base] = 1
		for _, candi := range m[base] {
			_, ok := visit[candi]
			if ok {
				continue
			}
			dfs(m, visit, candi, steps+1)
		}
		delete(visit, base)
	}

	dfs(m, visit, start, 0)
	return max

}
