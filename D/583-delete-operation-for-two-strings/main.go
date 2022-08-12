package main

func minDistance(word1 string, word2 string) int {

	cache := make([][]int, len(word1)+1)
	set := make([][]bool, len(word1)+1)
	for i := 0; i < len(word1)+1; i++ {
		cache[i] = make([]int, len(word2)+1)
		set[i] = make([]bool, len(word2)+1)
	}

	var recur func(idx1, idx2 int) int
	recur = func(idx1, idx2 int) int {
		if set[idx1][idx2] {
			return cache[idx1][idx2]
		}
		if idx1 == len(word1) {
			set[idx1][idx2] = true
			cache[idx1][idx2] = len(word2) - idx2
			return cache[idx1][idx2]
		}
		if idx2 == len(word2) {
			set[idx1][idx2] = true
			cache[idx1][idx2] = len(word1) - idx1
			return cache[idx1][idx2]
		}

		v1 := recur(idx1+1, idx2) + 1
		v := v1
		v2 := recur(idx1, idx2+1) + 1
		if v > v2 {
			v = v2
		}
		if word1[idx1] == word2[idx2] {
			v3 := recur(idx1+1, idx2+1)
			if v > v3 {
				v = v3
			}
		}

		set[idx1][idx2] = true
		cache[idx1][idx2] = v
		return cache[idx1][idx2]
	}

	recur(0, 0)
	return cache[0][0]
}
