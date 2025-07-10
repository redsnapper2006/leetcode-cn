package main

import (
	"fmt"
	"sort"
)

type Trie struct {
	N []*Trie
	C []string
}

func suggestedProducts(products []string, searchWord string) [][]string {
	root := &Trie{N: make([]*Trie, 26), C: []string{}}
	var build func(root *Trie, bb []byte, idx int)
	build = func(root *Trie, bb []byte, idx int) {
		if idx == len(bb) {
			return
		}

		off := bb[idx] - 'a'
		if root.N[off] == nil {
			root.N[off] = &Trie{N: make([]*Trie, 26), C: []string{}}
		}
		p := root.N[off]

		p.C = append(p.C, string(bb))
		sort.Strings(p.C)
		if len(p.C) > 3 {
			p.C = p.C[0:3]
		}

		build(root.N[off], bb, idx+1)
	}

	for _, p := range products {
		build(root, []byte(p), 0)
	}

	ans := [][]string{}
	p := root
	for _, b := range []byte(searchWord) {
		if p != nil && p.N[b-'a'] != nil {
			ans = append(ans, p.N[b-'a'].C)
			p = p.N[b-'a']
		} else {
			ans = append(ans, nil)
			p = nil
		}
	}
	return ans
}

func main() {
	fmt.Println(suggestedProducts([]string{
		"oenmjzunulkrjyxfugrpvkwoiwyxwgrweakhbswllbyziranhxkleggegegdailjgyteaghdqnjqdjfhyrapqmckvxgxmasnweej"},
		"oyrnqunojmtporeofgldjntqvlngobvtpbhmmdrkosxlkvmivonldjr"))
}
