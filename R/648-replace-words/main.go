package main

import (
	"fmt"
	"strings"
)

type Trie struct {
	IsEnd bool
	Next  []Trie
}

func replaceWords(dictionary []string, sentence string) string {
	trie := Trie{IsEnd: false, Next: make([]Trie, 26)}

	for _, words := range dictionary {
		var p *Trie = &trie
		for _, b := range words {
			if p.IsEnd {
				break
			}
			if len(p.Next) == 0 {
				p.Next = make([]Trie, 26)
			}
			p = &(p.Next[b-'a'])
		}
		if !p.IsEnd {
			p.IsEnd = true
		}
	}

	var ret []string
	arr := strings.Split(sentence, " ")
	for _, v := range arr {
		var p *Trie = &trie
		idx := -1
		isFound := false
		for i, b := range v {

			if p.IsEnd {
				isFound = true
				idx = i
				break
			}
			if len(p.Next) == 0 {
				break
			}
			p = &(p.Next[b-'a'])
		}
		if isFound {
			ret = append(ret, v[0:idx])
		} else {
			ret = append(ret, v)
		}
	}
	return strings.Join(ret, " ")
}

func main() {
	fmt.Println(replaceWords([]string{"cat", "bat", "rat"}, "the cattle was rattled by the battery"))
}
