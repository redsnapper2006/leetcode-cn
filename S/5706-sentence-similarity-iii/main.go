package main

import (
	"fmt"
	"strings"
)

func areSentencesSimilar(sentence1 string, sentence2 string) bool {
	s1 := strings.Split(sentence1, " ")
	s2 := strings.Split(sentence2, " ")
	if len(s1) == 0 || len(s2) == 0 {
		return true
	}
	if len(s1) == len(s2) {
		for i := 0; i < len(s1); i++ {
			if s1[i] != s2[i] {
				return false
			}
		}
		return true
	}
	g1 := s1
	g2 := s2
	if len(g1) < len(g2) {
		g1 = s2
		g2 = s1
	}
	ss1 := 0
	ss2 := 0
	isMatch := true
	unmatchCount := 0
	for ss1 < len(g1) && ss2 < len(g2) {
		if g1[ss1] == g2[ss2] {
			isMatch = true
			ss1++
			ss2++
			continue
		}
		if isMatch {
			unmatchCount++
			isMatch = false
		}
		ss1++
	}
	if ss2 < len(g2) {
		return false
	}
	if ss1 < len(g1) {
		unmatchCount++
	}
	return unmatchCount == 1
}

func main() {
	fmt.Println(areSentencesSimilar("a", "b"))
}
