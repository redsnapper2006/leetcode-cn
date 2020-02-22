package main

import (
	"fmt"
	"sort"
)

type ByteSlice []byte

func (p ByteSlice) Len() int {
	return len(p)
}

func (p ByteSlice) Swap(i, j int) {
	p[i], p[j] = p[j], p[i]
}

func (p ByteSlice) Less(i, j int) bool {
	return p[i] < p[j]
}

func groupAnagramsV2(strs []string) [][]string {
	buf := make(map[string][]string)

	for i := 0; i < len(strs); i++ {
		t := []byte(strs[i])
		sort.Sort(ByteSlice(t))
		buf[string(t)] = append(buf[string(t)], strs[i])
	}

	var r [][]string
	for _, v := range buf {
		r = append(r, v)
	}
	return r
}

func groupAnagrams(strs []string) [][]string {
	buf := make(map[[26]byte][]string)

	for i := 0; i < len(strs); i++ {
		var letters [26]byte
		for j := 0; j < len(strs[i]); j++ {
			letters[strs[i][j]-'a']++
		}
		buf[letters] = append(buf[letters], strs[i])
	}

	var r [][]string
	for _, v := range buf {
		r = append(r, v)
	}
	return r
}

func main() {
	fmt.Println(groupAnagrams([]string{"eat", "tea", "tan", "ate", "nat", "bat"}))
}
