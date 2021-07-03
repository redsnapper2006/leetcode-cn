package main

import (
	"fmt"
	"strconv"
	"strings"
)

func invalidTransactions(transactions []string) []string {
	ret := map[int]int{}
	m := map[string]map[string][]int{}
	m2 := map[string]map[string][]int{}
	for i := 0; i < len(transactions); i++ {
		trans := strings.Split(transactions[i], ",")
		v, _ := strconv.ParseInt(trans[2], 10, 64)
		account := int(v)
		_, ok := m[trans[0]]
		if !ok {
			m[trans[0]] = map[string][]int{}
			m2[trans[0]] = map[string][]int{}
		}
		_, ok2 := m[trans[0]][trans[3]]
		if !ok2 {
			m[trans[0]][trans[3]] = []int{}
			m2[trans[0]][trans[3]] = []int{}
		}
		v2, _ := strconv.ParseInt(trans[1], 10, 64)
		val := int(v2)
		m[trans[0]][trans[3]] = append(m[trans[0]][trans[3]], val)
		m2[trans[0]][trans[3]] = append(m2[trans[0]][trans[3]], i)
		if account > 1000 {
			ret[i]++
		}

		for k, v := range m[trans[0]] {
			if k == trans[3] {
				continue
			}
			for j := 0; j < len(v); j++ {
				diff := v[j] - val
				if diff < 0 {
					diff = -diff
				}
				if diff <= 60 {
					ret[i]++
					ret[m2[trans[0]][k][j]]++
				}
			}
		}

	}
	rr := []string{}
	for k := range ret {
		rr = append(rr, transactions[k])
	}
	return rr
}

func main() {
	fmt.Println()
}
