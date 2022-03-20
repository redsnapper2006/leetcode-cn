package main

func countHighestScoreNodes(parents []int) int {
	m := map[int]int{}
	pm := map[int][]int{}
	for i, p := range parents {

		m[i] = p
		if p == -1 {
			continue
		}
		_, ok := pm[p]
		if !ok {
			pm[p] = []int{}
		}
		pm[p] = append(pm[p], i)
	}
	candi := []int{}
	for i := 1; i < len(parents); i++ {
		_, ok := pm[i]
		if !ok {
			candi = append(candi, i)
		}
	}

	v := map[int]int{}
	for len(candi) > 0 {
		// fmt.Println(candi)
		tm := map[int]int{}
		for _, c := range candi {
			sum := 1
			isReady := true
			for _, k := range pm[c] {
				_, ok := v[k]
				if !ok {
					isReady = false
					break
				}
				sum += v[k]
			}
			if isReady {
				v[c] = sum
				if m[c] != -1 {
					tm[m[c]] = 1
				}
			}
		}
		t := []int{}
		for k := range tm {
			t = append(t, k)
		}
		candi = t
	}
	max := -1
	cnt := 0
	for i := 0; i < len(parents); i++ {
		aggr := 1
		for _, k := range pm[i] {
			if v[k] > 0 {
				aggr *= v[k]
			}
		}
		if i > 0 {
			aggr *= (v[0] - v[i])
		}
		if max < aggr {
			max = aggr
			cnt = 1
		} else if max == aggr {
			cnt++
		}
	}
	return cnt
}
