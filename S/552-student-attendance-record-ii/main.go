package main

import "fmt"

func checkRecord_DFS(n int) int {

	cached := make([][][]int, n)
	for i := 0; i < n; i++ {
		cached[i] = [][]int{{0, 0, 0}, {0, 0, 0}}
	}
	calced := make([][][]int, n)
	for i := 0; i < n; i++ {
		calced[i] = [][]int{{0, 0, 0}, {0, 0, 0}}
	}
	var dfs func(day int, ACnt int, LContinueCnt int) int
	dfs = func(day int, ACnt int, LContinueCnt int) int {
		if day == n {
			return 1
		}
		if calced[day][ACnt][LContinueCnt] == 1 {
			return cached[day][ACnt][LContinueCnt]
		}

		a := 0
		if ACnt < 1 {
			a = dfs(day+1, ACnt+1, 0)
		}
		l := 0
		if LContinueCnt < 2 {
			l = dfs(day+1, ACnt, LContinueCnt+1)
		}
		p := dfs(day+1, ACnt, 0)

		cached[day][ACnt][LContinueCnt] = (a + l + p) % 1000000007
		calced[day][ACnt][LContinueCnt] = 1
		return cached[day][ACnt][LContinueCnt]
	}
	dfs(0, 0, 0)
	ret := cached[0][0][0]
	ret %= 1000000007
	return ret
}

func checkRecord(n int) int {
	MOD := 1000000007

	a0l0, a0l1, a0l2, a1l0, a1l1, a1l2 := 1, 0, 0, 0, 0, 0
	// A 0 L 0
	// A 0 L 1
	// A 0 L 2
	// A 1 L 0
	// A 1 L 1
	// A 1 L 2

	for i := 1; i <= n; i++ {
		na0l0, na0l1, na0l2, na1l0, na1l1, na1l2 :=
			(a0l0+a0l1+a0l2)%MOD, a0l0, a0l1, (a0l0+a0l1+a0l2+a1l0+a1l1+a1l2)%MOD, a1l0, a1l1
		a0l0, a0l1, a0l2, a1l0, a1l1, a1l2 = na0l0, na0l1, na0l2, na1l0, na1l1, na1l2
	}

	return (a0l0 + a0l1 + a0l2 + a1l0 + a1l1 + a1l2) % MOD
}

func main() {
	fmt.Println()
}
