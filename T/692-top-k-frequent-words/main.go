package main

import (
	"fmt"
	"sort"
)

func topKFrequent(words []string, k int) []string {
	m := make(map[string]int)
	for i := 0; i < len(words); i++ {
		m[words[i]]++
	}

	freMap := make(map[int][]string)
	for k, v := range m {
		_, ok := freMap[v]
		if !ok {
			freMap[v] = []string{k}
		} else {
			freMap[v] = append(freMap[v], k)
		}
	}
	var fre []int
	for k, _ := range freMap {
		fre = append(fre, k)
	}

	sort.Ints(fre)
	count := 0
	var ret []string
	for i := len(fre) - 1; i >= 0; i-- {
		candis := freMap[fre[i]]
		sort.Strings(candis)
		c := len(candis)
		if count+len(candis) > k {
			c = k - count
		}
		count += c
		ret = append(ret, candis[0:c]...)
		if count == k {
			break
		}
	}
	return ret
}

func main() {
	fmt.Println("a")
}
