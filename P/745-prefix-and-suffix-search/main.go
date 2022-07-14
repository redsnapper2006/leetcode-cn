package main

import "fmt"

type Trie struct {
	I []int
	T []Trie
}

type WordFilter struct {
	P Trie
	S Trie
}

func Constructor(words []string) WordFilter {
	P := Trie{I: []int{}, T: make([]Trie, 26)}
	S := Trie{I: []int{}, T: make([]Trie, 26)}
	for i := 0; i < len(words); i++ {
		var p *Trie
		p = &P
		p.I = append(p.I, i)
		for j := 0; j < len(words[i]); j++ {
			offset := int(words[i][j] - byte('a'))
			p = &(p.T[offset])
			if len(p.I) == 0 {
				p.T = make([]Trie, 26)
			}
			p.I = append(p.I, i)
		}
		s := &S
		s.I = append(s.I, i)
		for j := len(words[i]) - 1; j >= 0; j-- {
			offset := int(words[i][j] - byte('a'))
			s = &(s.T[offset])
			if len(s.I) == 0 {
				s.T = make([]Trie, 26)
			}
			s.I = append(s.I, i)
		}
	}
	return WordFilter{P: P, S: S}
}

func (wf *WordFilter) F(prefix string, suffix string) int {
	p := &wf.P
	for i := 0; i < len(prefix); i++ {
		offset := int(prefix[i] - byte('a'))
		if len(p.T) == 0 {
			return -1
		}
		p = &(p.T[offset])
	}
	preIdx := p.I

	s := &wf.S
	for i := len(suffix) - 1; i >= 0; i-- {
		offset := int(suffix[i] - byte('a'))
		if len(s.T) == 0 {
			return -1
		}
		s = &(s.T[offset])
	}
	sufIdx := s.I
	idxP, idxS := len(preIdx)-1, len(sufIdx)-1

	for idxP >= 0 && idxS >= 0 {
		if preIdx[idxP] > sufIdx[idxS] {
			idxP--
		} else if preIdx[idxP] < sufIdx[idxS] {
			idxS--
		} else {
			return preIdx[idxP]
		}
	}
	return -1
}

func main() {
	o := Constructor([]string{"cabaabaaaa", "ccbcababac", "bacaabccba", "bcbbcbacaa", "abcaccbcaa", "accabaccaa", "cabcbbbcca", "ababccabcb", "caccbbcbab", "bccbacbcba"})
	fmt.Println(o.F("bccbacbcba", "a"))
	fmt.Println(o.F("ab", "abcaccbcaa"))
	fmt.Println(o.F("a", "aa"))
}
