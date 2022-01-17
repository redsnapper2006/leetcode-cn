package main

import "fmt"

func countVowelPermutation(n int) int {
	cache := make([][]int, n)
	for i := 0; i < n; i++ {
		cache[i] = make([]int, 5)
	}

	cache[0][0] = 1
	cache[0][1] = 1
	cache[0][2] = 1
	cache[0][3] = 1
	cache[0][4] = 1
	for i := 1; i < n; i++ {
		cache[i][0] = (cache[i-1][1]) % (1e9 + 7)
		cache[i][1] = (cache[i-1][0] + cache[i-1][2]) % (1e9 + 7)
		cache[i][2] = (cache[i-1][0] + cache[i-1][1] + cache[i-1][3] + cache[i-1][4]) % (1e9 + 7)
		cache[i][3] = (cache[i-1][2] + cache[i-1][4]) % (1e9 + 7)
		cache[i][4] = (cache[i-1][0]) % (1e9 + 7)
	}
	return (cache[n-1][0] + cache[n-1][1] + cache[n-1][2] + cache[n-1][3] + cache[n-1][4]) % (1e9 + 7)
}

func countVowelPermutation2(n int) int {
	cache := map[byte]map[int]int{}

	N := 1000000007
	var dfs func(n int, b byte) int
	dfs = func(n int, b byte) int {
		if n == 1 {
			return 1
		}
		// fmt.Println(string(b))
		_, ok := cache[b]
		if !ok {
			cache[b] = map[int]int{}
		}
		cv, ok2 := cache[b][n]
		if ok2 {
			return cv
		}

		v := -1
		if b == byte('a') {
			v = dfs(n-1, 'e') % N
		}
		if b == byte('e') {
			v = (dfs(n-1, 'a') + dfs(n-1, 'i')) % N
		}
		if b == byte('i') {
			v = (dfs(n-1, 'a') + dfs(n-1, 'e') + dfs(n-1, 'o') + dfs(n-1, 'u')) % N
		}
		if b == byte('o') {
			v = (dfs(n-1, 'i') + dfs(n-1, 'u')) % N
		}
		if b == byte('u') {
			v = dfs(n-1, 'a') % N
		}
		cache[b][n] = v
		return v
	}

	return (dfs(n, 'a') + dfs(n, 'e') + dfs(n, 'i') + dfs(n, 'o') + dfs(n, 'u')) % N

}

func main() {
	fmt.Println(countVowelPermutation(1))
	fmt.Println(countVowelPermutation(2))
	fmt.Println(countVowelPermutation(5))
}
