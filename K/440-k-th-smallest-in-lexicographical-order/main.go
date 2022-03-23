package main

import "fmt"

func findKthNumber(n int, k int) int {
	curr := 1
	k--

	steps := func(curr int, n int) int {
		first, last := curr, curr
		s := 0
		for first <= n {
			t := last
			if t > n {
				t = n
			}
			s += t - first + 1
			first = first * 10
			last = last*10 + 9
		}
		return s
	}

	for k > 0 {
		s := steps(curr, n)
		if s <= k {
			k -= s
			curr++
		} else {
			curr = curr * 10
			k--
		}
	}
	return curr

}

func findKthNumber2(n int, k int) int {

	var dfs func(n int, k *int, cur int) int
	dfs = func(n int, k *int, cur int) int {
		*k--
		if *k == 0 {
			return cur
		}

		for i := 0; i < 10; i++ {
			if cur*10+i > n {
				break
			}
			v := dfs(n, k, cur*10+i)
			if *k == 0 {
				return v
			}
		}
		return 0
	}

	kt := k
	for i := 1; i <= 9; i++ {
		v := dfs(n, &kt, i)
		if kt == 0 {
			return v
		}
	}

	return 0

}

func main() {
	fmt.Println(findKthNumber(100029, 16))
	fmt.Println(findKthNumber(957747794, 424238336))

}
