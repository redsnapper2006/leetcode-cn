package main

import (
	"fmt"
	"sort"
)

type DP struct {
	D int
	P int
}

type DPArr []DP

func (p DPArr) Len() int {
	return len(p)
}

func (p DPArr) Swap(i, j int) {
	p[i], p[j] = p[j], p[i]
}

func (p DPArr) Less(i, j int) bool {
	if p[i].D == p[j].D {
		return p[i].P < p[j].P
	}

	return p[i].D < p[j].D
}

func maxProfitAssignment(difficulty []int, profit []int, worker []int) int {
	var dp []DP
	for i := 0; i < len(difficulty); i++ {
		dp = append(dp, DP{D: difficulty[i], P: profit[i]})
	}
	sort.Sort(DPArr(dp))
	M := map[int]int{}
	p := -1
	for i := 0; i < len(dp); i++ {
		if dp[i].P > p {
			p = dp[i].P
		}
		M[dp[i].D] = p
	}
	buf := make([]int, 100001)

	for i := 1; i < len(buf); i++ {
		p, ok := M[i]
		if !ok {
			buf[i] = buf[i-1]
		} else {
			buf[i] = p
		}
	}
	sum := 0
	for _, w := range worker {
		sum += buf[w]
	}
	return sum
}

func main() {
	fmt.Println()
}
