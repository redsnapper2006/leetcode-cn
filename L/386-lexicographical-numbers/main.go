package main

import (
	"fmt"
	"sort"
)

type IntArr []int

func (p IntArr) Len() int {
	return len(p)
}

func (p IntArr) Swap(i, j int) {
	p[i], p[j] = p[j], p[i]
}

func (p IntArr) Less(i, j int) bool {
	a, b := make([]int, 6), make([]int, 6)
	pi, pj := p[i], p[j]
	// base := 10
	for i := 0; i < 6; i++ {
		a[i] = pi % 10
		b[i] = pj % 10
		pi /= 10
		pj /= 10
	}
	aIdx, bIdx := 5, 5
	for a[aIdx] == 0 {
		aIdx--
	}
	for b[bIdx] == 0 {
		bIdx--
	}
	for aIdx >= 0 && bIdx >= 0 {
		if a[aIdx] == b[bIdx] {
			aIdx--
			bIdx--
			continue
		} else {
			return a[aIdx] < b[bIdx]
		}
	}
	return bIdx > aIdx
}

func lexicalOrderV2(n int) []int {
	buf := make([]int, n)
	for i := 0; i < n; i++ {
		buf[i] = i + 1
	}
	sort.Sort(IntArr(buf))
	return buf
}

func lexicalOrder(n int) []int {
	var buf []int
	var dfs func(c, n int, buf *[]int)
	dfs = func(c, n int, buf *[]int) {
		for i := 0; i <= 9; i++ {
			if c*10+i > n {
				break
			}
			*buf = append(*buf, c*10+i)
			dfs(c*10+i, n, buf)
		}
	}
	for i := 1; i <= 9; i++ {
		if i > n {
			break
		}
		buf = append(buf, i)
		dfs(i, n, &buf)
	}
	return buf
}

func main() {
	fmt.Println(lexicalOrder(13))
}
