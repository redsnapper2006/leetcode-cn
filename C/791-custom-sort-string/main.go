package main

import (
	"fmt"
	"sort"
)

type CustSortByte struct {
	B []byte
	D []int
}

func (p CustSortByte) Len() int {
	return len(p.B)
}

func (p CustSortByte) Swap(i, j int) {
	p.B[i], p.B[j] = p.B[j], p.B[i]
}

func (p CustSortByte) Less(i, j int) bool {
	return p.D[p.B[i]-byte('a')] > p.D[p.B[j]-byte('a')]
}

func customSortString(S string, T string) string {
	idx := make([]int, 26)
	for i := 0; i < len(S); i++ {
		idx[S[i]-byte('a')] = len(S) - i
	}
	csb := CustSortByte{B: []byte(T), D: idx}
	sort.Sort(csb)
	return string(csb.B)
}

func main() {
	fmt.Println()
}
