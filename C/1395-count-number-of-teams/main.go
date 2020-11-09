package main

import "fmt"

func numTeams(rating []int) int {
	ret := 0
	ascBuf := []int{rating[0]}
	M := map[int]int{rating[0]: 0}
	for i := 1; i < len(rating); i++ {
		s, e := 0, len(ascBuf)-1
		for s <= e {
			m := s + (e-s)/2
			if ascBuf[m] > rating[i] {
				e = m - 1
			} else {
				s = m + 1
			}
		}
		if s >= len(ascBuf) {
			ascBuf = append(ascBuf, rating[i])
		} else {
			t := make([]int, len(ascBuf)+1)
			copy(t, ascBuf[0:s])
			t[s] = rating[i]
			copy(t[s+1:], ascBuf[s:])
			ascBuf = t
		}
		M[rating[i]] = s
		for j := 0; j < s; j++ {
			ret += M[ascBuf[j]]
		}
	}
	// fmt.Println(ascBuf)

	descBuf := []int{rating[len(rating)-1]}
	M2 := map[int]int{rating[len(rating)-1]: 0}
	for i := len(rating) - 2; i >= 0; i-- {
		s, e := 0, len(descBuf)-1
		for s <= e {
			m := s + (e-s)/2
			if descBuf[m] > rating[i] {
				e = m - 1
			} else {
				s = m + 1
			}
		}
		if s >= len(descBuf) {
			descBuf = append(descBuf, rating[i])
		} else {
			t := make([]int, len(descBuf)+1)
			copy(t, descBuf[0:s])
			t[s] = rating[i]
			copy(t[s+1:], descBuf[s:])
			descBuf = t
		}
		M2[rating[i]] = s
		for j := 0; j < s; j++ {
			ret += M2[descBuf[j]]
		}
	}
	// fmt.Println(descBuf)
	return ret
}

func main() {
	fmt.Println()
}
