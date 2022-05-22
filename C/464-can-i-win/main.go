package main

func canIWin(maxChoosableInteger int, desiredTotal int) bool {
	if desiredTotal == 0 {
		return true
	}
	if maxChoosableInteger*(maxChoosableInteger+1)/2 < desiredTotal {
		return false
	}

	cache := map[int]bool{}

	var dfs func(visited int, steps int, sum int) bool
	dfs = func(visited int, steps int, sum int) bool {
		cacheResult, ok := cache[visited]
		if ok {
			return cacheResult
		}

		if sum >= desiredTotal {
			result := steps%2 == 1
			cache[visited] = result
			return result
		}

		if steps >= maxChoosableInteger {
			result := sum >= desiredTotal && steps%2 == 1
			cache[visited] = result
			return result
		}
		var result bool
		for i := 1; i <= maxChoosableInteger; i++ {
			if visited&(1<<i) > 0 {
				continue
			}
			result = dfs(visited|(1<<i), steps+1, sum+i)
			if result && steps%2 == 0 || !result && steps%2 == 1 {
				break
			}
		}
		cache[visited] = result
		return result
	}

	return dfs(0, 0, 0)
}
