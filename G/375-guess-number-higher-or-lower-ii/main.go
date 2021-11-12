package main

import "fmt"

func getMoneyAmount(n int) int {
	m := map[int]map[int]int{}

	var recur func(m map[int]map[int]int, start, end int) int
	recur = func(m map[int]map[int]int, start, end int) int {
		_, ok := m[start]
		if !ok {
			m[start] = map[int]int{}
		}

		if start == end {
			m[start][end] = 0
			return 0
		}
		if start+1 == end {
			m[start][end] = start
			return start
		}
		if start+2 == end {
			m[start][end] = start + 1
			return start + 1
		}

		min := (n + 1) * n / 2
		for i := start + 2; i <= end-1; i++ {
			left, right := 0, 0
			_, ok := m[start]
			if !ok {
				left = recur(m, start, i-1)
			} else {
				_, ok2 := m[start][i-1]
				if !ok2 {
					left = recur(m, start, i-1)
				} else {
					left = m[start][i-1]
				}
			}

			_, ok3 := m[i+1]
			if !ok3 {
				right = recur(m, i+1, end)
			} else {
				_, ok4 := m[i+1][end]
				if !ok4 {
					right = recur(m, i+1, end)
				} else {
					right = m[i+1][end]
				}
			}

			v := left
			if v < right {
				v = right
			}
			if min > v+i {
				min = v + i
			}
		}
		m[start][end] = min
		return min
	}
	recur(m, 1, n)
	return m[1][n]
}

func main() {
	fmt.Println()
}
