package main

import (
	"fmt"
	"sort"
)

func deckRevealedIncreasing(deck []int) []int {
	sort.Ints(deck)
	if len(deck) <= 2 {
		return deck
	}

	buf := []int{deck[len(deck)-1]}
	idx := len(deck) - 2
	for idx > 0 {
		buf = append(buf, deck[idx])
		t := buf[0]
		buf = buf[1:]
		buf = append(buf, t)
		idx--
	}
	buf = append(buf, deck[idx])
	s, e := 0, len(buf)-1
	for s < e {
		buf[s], buf[e] = buf[e], buf[s]
		s++
		e--
	}
	return buf
}

func main() {
	fmt.Println(deckRevealedIncreasing([]int{17, 13, 11, 2, 3, 5, 7}))
}
