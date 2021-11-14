package main

import "fmt"

type MapSum struct {
	N []MapSum
	S []int
}

func Constructor() MapSum {
	return MapSum{N: nil, S: nil}
}

func (this *MapSum) Insert(key string, val int) {
	p := this
	var pp *MapSum = nil
	offset := -1
	for _, b := range key {
		if len(p.N) == 0 {
			p.N = make([]MapSum, 26)
			p.S = make([]int, 26)
		}
		offset = int(byte(b) - byte('a'))
		pp = p
		p = &p.N[offset]
	}
	pp.S[offset] = val
}

func (this *MapSum) Sum(prefix string) int {
	p := this
	offset := -1
	v := 0
	prev := -1
	for _, b := range prefix {
		if len(p.N) == 0 {
			return 0
		}
		offset = int(byte(b) - byte('a'))
		prev = p.S[offset]
		p = &p.N[offset]
	}
	v = prev

	var recur func(p *MapSum)
	recur = func(p *MapSum) {
		if len(p.N) == 0 {
			return
		}
		for i := 0; i < len(p.N); i++ {
			v += p.S[i]
			recur(&p.N[i])
		}
	}
	recur(p)
	return v
}

func main() {
	fmt.Println()
}
