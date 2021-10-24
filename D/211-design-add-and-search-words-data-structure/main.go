package main

import "fmt"

type WordDictionary struct {
	WD     []*WordDictionary
	IsWord bool
}

func Constructor() WordDictionary {
	return WordDictionary{WD: make([]*WordDictionary, 26), IsWord: false}
}

func (this *WordDictionary) AddWord(word string) {
	p := this
	for _, b := range word {
		offset := int(byte(b) - byte('a'))
		if p.WD[offset] == nil {
			p.WD[offset] = &WordDictionary{WD: make([]*WordDictionary, 26), IsWord: false}
		}
		p = p.WD[offset]
	}
	p.IsWord = true
}

func (this *WordDictionary) Search(word string) bool {
	p := []*WordDictionary{this}
	for _, b := range word {
		t := []*WordDictionary{}
		if byte(b) == byte('.') {
			for _, sp := range p {
				for i := 0; i < len(sp.WD); i++ {
					if sp.WD[i] != nil {
						t = append(t, sp.WD[i])
					}
				}
			}
		} else {
			offset := int(byte(b) - byte('a'))
			for _, sp := range p {
				if sp.WD[offset] != nil {
					t = append(t, sp.WD[offset])
				}
			}
		}
		p = t
		if len(p) == 0 {
			return false
		}
	}
	for _, sp := range p {
		if sp.IsWord {
			return true
		}
	}
	return false
}

func main() {
	fmt.Println()
}
