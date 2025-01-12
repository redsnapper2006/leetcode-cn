package main

func lenLongestFibSubseq(arr []int) int {
	idx := map[int]int{}
	for i := 0; i < len(arr); i++ {
		idx[arr[i]] = i
	}

	max := 0
	var dfs func(i, j, step int)
	dfs = func(i, j, step int) {
		_, ok := idx[i+j]
		if !ok {
			if step > max {
				max = step
			}
			return
		}
		dfs(j, i+j, step+1)
	}

	for i := 0; i < len(arr)-2; i++ {
		for j := i + 1; j < len(arr)-1; j++ {
			dfs(arr[i], arr[j], 2)
		}
	}
	if max < 3 {
		return 0
	}
	return max
}
