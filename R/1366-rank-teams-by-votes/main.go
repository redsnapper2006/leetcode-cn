package main

import (
	"fmt"
	"sort"
)

type R struct {
	L byte
	P []int
}

type RArr []R

func (p RArr) Len() int {
	return len(p)
}

func (p RArr) Swap(i, j int) {
	p[i], p[j] = p[j], p[i]
}

func (p RArr) Less(i, j int) bool {
	for m := 0; m < len(p[i].P); m++ {
		if p[i].P[m] == p[j].P[m] {
			continue
		}
		return p[i].P[m] > p[j].P[m]
	}

	return p[i].L < p[j].L
}

func rankTeams(votes []string) string {
	if len(votes) == 1 {
		return votes[0]
	}
	buf := make([]R, 26)
	for i := 0; i < 26; i++ {
		buf[i] = R{L: byte(i + 'A'), P: make([]int, len(votes[0]))}
	}
	for _, v := range votes {
		for i, b := range v {
			buf[int(b-'A')].P[i]++
		}
	}
	sort.Sort(RArr(buf))
	var b []byte
	for i := 0; i < len(votes[0]); i++ {
		b = append(b, buf[i].L)
	}
	return string(b)
}

func main() {
	fmt.Println()
}
