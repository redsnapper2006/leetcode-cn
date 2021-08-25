package main

import (
	"fmt"
	"strings"
)

type Trie struct {
	Finish bool
	Child  []*Trie
}

func replaceWords(dictionary []string, sentence string) string {
	root := Trie{Finish: false, Child: make([]*Trie, 26)}
	for _, word := range dictionary {
		p := &root
		for _, b := range word {
			idx := int(b - 'a')
			if p.Child[idx] == nil {
				p.Child[idx] = &Trie{Finish: false, Child: make([]*Trie, 26)}
			}
			p = p.Child[idx]
		}
		p.Finish = true
	}

	ret := []string{}
	arr := strings.Split(sentence, " ")
	for _, a := range arr {
		p := &root
		isSucc := false
		bArr := []byte{}
		for _, b := range a {
			if p.Finish {
				isSucc = true
				break
			}
			if p.Child[int(b-'a')] != nil {
				bArr = append(bArr, byte(b))
				p = p.Child[int(b-'a')]
			} else {
				break
			}
		}
		if isSucc {
			ret = append(ret, string(bArr))
		} else {
			ret = append(ret, a)
		}
	}
	return strings.Join(ret, " ")
}

func main() {
	fmt.Println()
}
