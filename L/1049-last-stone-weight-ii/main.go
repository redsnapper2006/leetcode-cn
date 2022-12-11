package main

func lastStoneWeightII(stones []int) int {
	ret := 1<<31 - 1

	var dfs func(stones []int, idx, sum1, sum2 int)
	dfs = func(stones []int, idx, sum1, sum2 int) {
		if idx == len(stones) {
			diff := sum1 - sum2
			if diff < 0 {
				diff = -diff
			}
			if ret > diff {
				ret = diff
			}
			return
		}

		dfs(stones, idx+1, sum1+stones[idx], sum2)
		dfs(stones, idx+1, sum1, sum2+stones[idx])
	}

	dfs(stones, 1, stones[0], 0)
	return ret
}
