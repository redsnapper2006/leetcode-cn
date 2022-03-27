package main

func missingRolls(rolls []int, mean int, n int) []int {
	sum := 0
	for _, r := range rolls {
		sum += r
	}

	diff := mean*(n+len(rolls)) - sum
	if diff < 1*n || diff > 6*n {
		return []int{}
	}

	mod := diff % n
	base := diff / n
	minus := 0
	if mod > 0 {
		base++
		minus = base*n - diff
	}

	ret := []int{}
	for i := 0; i < n-minus; i++ {
		ret = append(ret, base)
	}

	for i := 0; i < minus; i++ {
		ret = append(ret, base-1)
	}

	return ret

}
