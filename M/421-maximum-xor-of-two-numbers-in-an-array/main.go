package main

import "fmt"

type Trie struct {
	L []*Trie
}

func findMaximumXOR(nums []int) int {
	if len(nums) == 1 {
		return 0
	}

	N := 2
	H := Trie{L: make([]*Trie, N)}
	p := &H
	base := nums[0]
	constructor := func(T *Trie, n int) {
		p := T
		for i := 30; i >= 0; i-- {
			idx := 0
			if (n & (1 << i)) > 0 {
				idx = 1
			}

			if p.L[idx] == nil {
				t := Trie{L: make([]*Trie, N)}
				p.L[idx] = &t
			}
			p = p.L[idx]
		}
	}
	constructor(p, base)

	max := 0
	for i := 1; i < len(nums); i++ {
		p = &H
		base := nums[i]
		constructor(p, base)
		accum := 0
		for i := 30; i >= 0; i-- {
			idx := 0
			xor := 1
			if (base & (1 << i)) > 0 {
				idx = 1
				xor = 0
			}
			next := xor
			if p.L[xor] == nil {
				next = idx
			}
			p = p.L[next]
			if next != idx {
				accum += 1 << i
			}
		}
		if accum > max {
			max = accum
		}
	}
	return max
}

func main() {
	fmt.Println(findMaximumXOR([]int{3, 10, 5, 25, 2, 8}))
}
